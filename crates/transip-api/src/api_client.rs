use core::time::Duration;

use serde::{de::DeserializeOwned, Serialize};
use tracing::info;
use ureq::{serde_json, Agent, AgentBuilder};

use crate::authentication::{
    AuthRequest, TokenResponse, UrlAuthentication, KeyPair, Token, TokenExpiration, TokenExpired,
};
use crate::{Configuration, Error, Result};

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/";
const TOKEN_EXPIRATION_TIME: TokenExpiration = TokenExpiration::Seconds(120);
const AGENT_TIMEOUT_SECONDS: u64 = 30;

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

/// ApiClient is the main entry for this library. Creation is done by using ApiClient::new().
/// After creation of a client, this client can be used to call a Transip API call.
/// Each call starts with a check to see if we have a valid JWT token
/// If the token is expired or non existant then the Transip API call for requesting a new token is called
/// Tokens are persisted to disk on exit and reused if not expired on application startup
pub struct ApiClient {
    pub(crate) url: Url,
    configuration: Box<dyn Configuration>,
    key: Option<KeyPair>,
    agent: Agent,
    token: Option<Token>,
}

#[cfg(test)]
impl ApiClient {
    pub fn demo() -> Self {
        ApiClient {
            url: TRANSIP_API_PREFIX.into(),
            key: None,
            agent: AgentBuilder::new()
                .timeout(Duration::from_secs(AGENT_TIMEOUT_SECONDS))
                .build(),
            token: Some(Token::demo()),
            configuration: crate::environment::demo_configuration(),
        }
    }
}

impl TryFrom<Box<dyn Configuration>> for ApiClient {
    type Error = Error;
    fn try_from(configuration: Box<dyn Configuration>) -> Result<Self> {
        KeyPair::try_from_file(configuration.private_key_pem_file())
            .map(|key| ApiClient {
                url: TRANSIP_API_PREFIX.into(),
                key: Some(key),
                agent: AgentBuilder::new()
                    .timeout(Duration::from_secs(AGENT_TIMEOUT_SECONDS))
                    .build(),
                token: Token::try_from_file(configuration.token_path()).ok(),
                configuration,
            })
    }
}

#[cfg(not(test))]
impl Drop for ApiClient {
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            if let Err(error) = std::fs::write(self.configuration.token_path(), token.raw()) {
                tracing::error!("Error {} writing token to {}", error, self.configuration.token_path());
            }
        }
    }
}

impl ApiClient {
    fn refresh_token_if_needed(&mut self) -> Result<()> {
        if self.token.token_expired() {
            info!("New or refresh token needed");
            let auth_request = AuthRequest::new(
                &self.configuration.user_name(),
                &TOKEN_EXPIRATION_TIME.to_string(),
            );
            let json = auth_request.json();
            let signature = self.key.as_ref().unwrap().sign(&json)?;
            tracing::info!("Json signing success");
            let token_response = self
                .agent
                .post(&self.url.auth())
                .set("Content-Type", "application/json")
                .set("Signature", &signature)
                .send_bytes(json.as_slice())
                .map_err(Box::new)?
                .into_json::<TokenResponse>()?;
            self.token = Some(Token::try_from(token_response.token)?);
            info!("New or refresh token succeeded");
        }
        Ok(())
    }

    pub(crate) fn get<T>(&mut self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.refresh_token_if_needed()?;
        let token = self.token.as_ref().ok_or(Error::Token)?;
        info!("get {} calling", url);
        let json = self
            .agent
            .get(url)
            .set("Authorization", &format!("Bearer {}", token.raw()))
            .call()
            .map_err(Box::new)?
            .into_json::<T>()?;
        info!("get {} called successfully", url);
        Ok(json)
    }

    pub(crate) fn delete<T>(&mut self, url: &str, t: T) -> Result<()>
    where
        T: Serialize,
    {
        self.refresh_token_if_needed()?;
        let token = self.token.as_ref().ok_or(Error::Token)?;
        info!("delete {} calling", url);
        self.agent
            .delete(url)
            .set("Authorization", &format!("Bearer {}", token.raw()))
            .send_json(t)
            .map_err(Box::new)?;
        info!("delete {} called successfully", url);
        Ok(())
    }

    pub(crate) fn post<T>(&mut self, url: &str, t: T) -> Result<()>
    where
        T: Serialize,
    {
        self.refresh_token_if_needed()?;
        let token = self.token.as_ref().ok_or(Error::Token)?;
        info!("post {} calling with {}", url, serde_json::to_string(&t)?);
        self.agent
            .post(url)
            .set("Authorization", &format!("Bearer {}", token.raw()))
            .send_json(t)
            .map_err(Box::new)?;
        info!("post {} called successfully", url);
        Ok(())
    }
}

