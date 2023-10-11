use core::fmt::Display;
use core::time::Duration;

use std::fs::File;
use std::{io::BufReader, path::Path};

use chrono::Utc;
use ring::signature::{self, RsaKeyPair};
use serde::{de::DeserializeOwned, Serialize};
use tracing::info;
use ureq::{Agent, AgentBuilder};

use crate::authentication::{
    sign, token_expiration_timestamp, AuthRequest, TokenResponse, UrlAuthentication,
};
use crate::{Configuration, Error, Result};

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/";
const TOKEN_EXPIRATION_TIME: TokenExpiration = TokenExpiration::Minutes(5);
const AGENT_TIMEOUT_SECONDS: u64 = 30;

trait TokenExpired {
    fn token_expired(&self) -> bool;
}

#[allow(dead_code)]
pub enum TokenExpiration {
    Seconds(u8),
    Minutes(u8),
    Hours(u8),
}

fn fmt(count: &u8, unit: &'static str) -> String {
    format!("{} {}", count, unit) + if count <= &1 { "" } else { "s" }
}

impl Display for TokenExpiration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TokenExpiration::Seconds(seconds) => fmt(seconds, "second"),
                TokenExpiration::Minutes(minutes) => fmt(minutes, "minute"),
                TokenExpiration::Hours(hours) => fmt(hours, "hour"),
            }
        )
    }
}

pub fn read_rsa_key_pair_from_pem<P>(path: P) -> Result<RsaKeyPair>
where
    P: AsRef<Path>,
{
    let file = File::open(path.as_ref())?;
    info!("Key file {} opened", path.as_ref().to_string_lossy());
    let mut buf_reader = BufReader::new(file);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut buf_reader)?;
    if keys.is_empty() {
        Err(Error::Key("None"))
    } else {
        signature::RsaKeyPair::from_pkcs8(keys[0].as_slice()).map_err(|_| Error::Key("Invalid"))
    }
}

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

struct Token {
    raw: String,
    expired: i64,
}

/// ApiClient is the main entry for this library. Creation is done by using ApiClient::new().
/// After creation of a client, this client can be used to call a Transip API call.
/// Each call starts with a check to see if we have a valid JWT token
/// If the token is expired or non existant then the Transip API call for requesting a new token is called
/// Tokens are persisted to disk on exit and reused if not expired on application startup
pub struct ApiClient {
    pub(crate) url: Url,
    configuration: Box<dyn Configuration>,
    key: RsaKeyPair,
    // token: Option<Token>,
    agent: Agent,
    token: Option<Token>,
}

impl TokenExpired for Token {
    fn token_expired(&self) -> bool {
        self.expired < Utc::now().timestamp() + 2
    }
}

impl TokenExpired for Option<Token> {
    fn token_expired(&self) -> bool {
        if self.is_some() {
            self.as_ref().unwrap().token_expired()
        } else {
            true
        }
    }
}

impl TryFrom<Box<dyn Configuration>> for ApiClient {
    type Error = Error;
    fn try_from(configuration: Box<dyn Configuration>) -> std::result::Result<Self, Self::Error> {
        read_rsa_key_pair_from_pem(configuration.private_key()).map(|key| ApiClient {
            url: TRANSIP_API_PREFIX.into(),
            key,
            configuration,
            agent: AgentBuilder::new()
                .timeout(Duration::from_secs(AGENT_TIMEOUT_SECONDS))
                .build(),
            token: None,
        })
    }
}

impl ApiClient {
    fn refresh_token_if_needed(&mut self) -> Result<()> {
        if self.token.token_expired() {
            let auth_request = AuthRequest::new(
                &self.configuration.user_name(),
                &TOKEN_EXPIRATION_TIME.to_string(),
            );
            let json = auth_request.json();
            let signature = sign(json.as_slice(), &self.key)?;
            tracing::info!("Json signing success");
            let token_response = self
                .agent
                .post(&self.url.auth())
                .set("Signature", &signature)
                .send_bytes(json.as_slice())
                .map_err(Box::new)?
                .into_json::<TokenResponse>()?;
            let timestamp = token_expiration_timestamp(&token_response.token)?;
            self.token = Some(Token {
                raw: token_response.token,
                expired: timestamp,
            });
        }
        Ok(())
    }

    pub(crate) fn get<T>(&mut self, url: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.refresh_token_if_needed()?;
        let token = self.token.as_ref().ok_or(Error::Token)?;
        let json = self
            .agent
            .get(url)
            .set("Authorization", &format!("Bearer {}", &token.raw))
            .call()
            .map_err(Box::new)?
            .into_json::<T>()?;
        Ok(json)
    }

    pub(crate) fn delete<T>(&mut self, url: &str, t: T) -> Result<()>
    where
        T: Serialize,
    {
        self.refresh_token_if_needed()?;
        let token = self.token.as_ref().ok_or(Error::Token)?;
        self.agent
            .delete(url)
            .set("Authorization", &format!("Bearer {}", &token.raw))
            .send_json(t)
            .map_err(Box::new)?;
        Ok(())
    }

    pub(crate) fn post<T>(&mut self, url: &str, t: T) -> Result<()>
    where
        T: Serialize,
    {
        self.refresh_token_if_needed()?;
        let token = self.token.as_ref().ok_or(Error::Token)?;
        self.agent
            .post(url)
            .set("Authorization", &format!("Bearer {}", &token.raw))
            .send_json(t)
            .map_err(Box::new)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Token, TokenExpired};
    use chrono::Utc;

    #[test]
    fn expired_if_none() {
        let token: Option<Token> = None;
        assert!(token.token_expired());
    }

    #[test]
    fn expired_if_some() {
        let token: Option<Token> = Some(Token {
            raw: Default::default(),
            expired: Utc::now().timestamp(),
        });
        assert!(token.token_expired());
    }

    #[test]
    fn not_expired_if_some() {
        let token: Option<Token> = Some(Token {
            raw: Default::default(),
            expired: Utc::now().timestamp() + 10,
        });
        assert!(!token.token_expired());
    }
}
