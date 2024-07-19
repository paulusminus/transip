use core::time::Duration;
use std::fmt::Debug;
use std::net::{SocketAddr, ToSocketAddrs};

use serde::{de::DeserializeOwned, Serialize};
use tracing::{info, instrument};
use ureq::{Agent, AgentBuilder, Resolver};

use crate::authentication::{
    AuthRequest, KeyPair, Token, TokenExpired, TokenResponse, UrlAuthentication,
};
use crate::error::ResultExt;
use crate::{Configuration, Error, Result};

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/";
const AGENT_TIMEOUT_SECONDS: u64 = 30;
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), " ", env!("CARGO_PKG_VERSION"));

macro_rules! timeit {
    ($url:expr, $method:expr, $code:block) => {{
        let start = std::time::Instant::now();
        let t = $code;
        if t.is_err() {
            tracing::error!(
                "error {} {} after {} milliseconds",
                $method,
                $url,
                start.elapsed().as_millis()
            );
        } else {
            tracing::info!(
                "result {} {} after {} milliseconds",
                $method,
                $url,
                start.elapsed().as_millis()
            );
        };
        t
    }};
}

#[derive(Debug)]
pub struct Url {
    pub prefix: String,
}

impl From<&str> for Url {
    fn from(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_owned(),
        }
    }
}

/// Client is the main entry for this library. Creation is done by using Client::try_from(configuration).
/// After creation of a client, this client can be used to call a Transip API call.
/// Each call starts with a check to see if we have a valid JWT token
/// If the token is expired or non existant then the Transip API call for requesting a new token is called
/// Tokens are persisted to disk on exit and reused if not expired on application startup
pub struct Client {
    pub(crate) url: Url,
    configuration: Box<dyn Configuration>,
    key: Option<KeyPair>,
    agent: Agent,
    token: Option<Token>,
}

impl Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url.prefix)
    }
}

// #[cfg(test)]
impl Client {
    pub fn demo() -> Self {
        Self {
            url: TRANSIP_API_PREFIX.into(),
            key: None,
            agent: AgentBuilder::new()
                .timeout(Duration::from_secs(AGENT_TIMEOUT_SECONDS))
                .user_agent(USER_AGENT)
                .build(),
            token: Some(Token::demo()),
            configuration: crate::environment::demo_configuration(),
        }
    }

    pub fn test(prefix: String) -> Self {
        Self {
            url: format!("{prefix}/").as_str().into(),
            key: None,
            agent: AgentBuilder::new()
                .timeout(Duration::from_secs(AGENT_TIMEOUT_SECONDS))
                .user_agent(USER_AGENT)
                .build(),
            token: Some(Token::demo()),
            configuration: crate::environment::demo_configuration(),
        }
    }
}

pub struct Ipv6Resolver;

impl Resolver for Ipv6Resolver {
    fn resolve(&self, netloc: &str) -> std::io::Result<Vec<std::net::SocketAddr>> {
        ToSocketAddrs::to_socket_addrs(netloc).map(|iter| {
            iter
                // only keep ipv6 addresses
                .filter(|s| s.is_ipv6())
                .collect::<Vec<SocketAddr>>()

        })
        .inspect(|v| {
            if v.is_empty() {
                info!("Failed to find any ipv6 addresses. This probably means \
                    the DNS server didn't return any.")
            }
        })
    }
}

fn build_agent(ipv6only: bool) -> AgentBuilder {
    if ipv6only {
        AgentBuilder::new().resolver(Ipv6Resolver {})
    }
    else {
        AgentBuilder::new()
    }
}

impl TryFrom<Box<dyn Configuration>> for Client {
    type Error = Error;
    fn try_from(configuration: Box<dyn Configuration>) -> Result<Self> {
        KeyPair::try_from_file(configuration.private_key_pem_file()).map(|key| Self {
            url: TRANSIP_API_PREFIX.into(),
            key: Some(key),
            agent: build_agent(configuration.ipv6_only())
                .user_agent(USER_AGENT)
                .timeout(Duration::from_secs(AGENT_TIMEOUT_SECONDS))
                .build(),
            token: Token::try_from_file(configuration.token_path()).ok(),
            configuration,
        })
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        if self.key.is_some() {
            if let Some(token) = self.token.take() {
                if let Err(error) = token.try_to_write_file(self.configuration.token_path()) {
                    tracing::error!(
                        "Error {} writing token to {}",
                        error,
                        self.configuration.token_path()
                    );
                }
            }
        }
    }
}

impl Client {
    fn refresh_token_if_needed(&mut self) -> Result<()> {
        if self.token.token_expired() {
            let span = tracing::span!(tracing::Level::INFO, "token_refresh");
            let _span_enter = span.enter();
            let token_result = timeit!(&self.url.auth(), "POST", {
                let auth_request = AuthRequest::new(
                    self.configuration.user_name(),
                    self.configuration.token_expiration(),
                    self.configuration.read_only(),
                    self.configuration.whitelisted_only(),
                );
                let json = auth_request.json();
                let signature = self.key.as_ref().unwrap().sign(&json)?;
                let token_response = self
                    .agent
                    .post(&self.url.auth())
                    .set("Content-Type", "application/json")
                    .set("Signature", &signature)
                    .send_bytes(json.as_slice())
                    .map_err(Box::new)?
                    .into_json::<TokenResponse>()?;
                Token::try_from(token_response.token)
            });
            self.token = token_result.ok();
        }
        Ok(())
    }

    #[instrument(skip(self))]
    pub(crate) fn get<T>(&mut self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        timeit!(url, "GET", {
            self.refresh_token_if_needed()?;
            let token = self.token.as_ref().ok_or(Error::Token)?;
            self.agent
                .get(url)
                .set("Authorization", &format!("Bearer {}", token.raw()))
                .call()
                .map_err(Box::new)?
                .into_json::<T>()
                .err_into()
        })
    }

    #[instrument(skip(self))]
    pub(crate) fn delete<T>(&mut self, url: &str, object: T) -> Result<()>
    where
        T: Serialize + Debug,
    {
        timeit!(url, "DELETE", {
            self.refresh_token_if_needed()?;
            let token = self.token.as_ref().ok_or(Error::Token)?;
            self.agent
                .delete(url)
                .set("Authorization", &format!("Bearer {}", token.raw()))
                .send_json(object)
                .map_err(Box::new)?;
            Ok(())
        })
    }

    #[instrument(skip(self))]
    pub(crate) fn delete_no_object(&mut self, url: &str) -> Result<()> {
        timeit!(url, "DELETE", {
            self.refresh_token_if_needed()?;
            let token = self.token.as_ref().ok_or(Error::Token)?;
            self.agent
                .delete(url)
                .set("Authorization", &format!("Bearer {}", token.raw()))
                .call()
                .map_err(Box::new)?;
            Ok(())
        })
    }

    #[instrument(skip(self))]
    pub(crate) fn patch<T>(&mut self, url: &str, object: T) -> Result<()>
    where
        T: Serialize + Debug,
    {
        timeit!(url, "PATCH", {
            self.refresh_token_if_needed()?;
            let token = self.token.as_ref().ok_or(Error::Token)?;
            self.agent
                .patch(url)
                .set("Authorization", &format!("Bearer {}", token.raw()))
                .send_json(object)
                .map_err(Box::new)?;
            Ok(())
        })
    }

    #[instrument(skip(self))]
    pub(crate) fn post<T>(&mut self, url: &str, body: T) -> Result<()>
    where
        T: Serialize + Debug,
    {
        timeit!(url, "POST", {
            self.refresh_token_if_needed()?;
            let token = self.token.as_ref().ok_or(Error::Token)?;
            self.agent
                .post(url)
                .set("Authorization", &format!("Bearer {}", token.raw()))
                .send_json(body)
                .map_err(Box::new)?;
            Ok(())
        })
    }

    #[instrument(skip(self))]
    pub(crate) fn put<T>(&mut self, url: &str, body: T) -> Result<()>
    where
        T: Serialize + Debug,
    {
        timeit!(url, "PUT", {
            self.refresh_token_if_needed()?;
            let token = self.token.as_ref().ok_or(Error::Token)?;
            self.agent
                .put(url)
                .set("Authorization", &format!("Bearer {}", token.raw()))
                .send_json(body)
                .map_err(Box::new)?;
            Ok(())
        })
    }
}
