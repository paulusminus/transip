use core::fmt::Display;
use core::str::from_utf8;
use core::time::Duration;

use std::fs::File;
use std::path::PathBuf;
use std::{fs::read_dir, ffi::OsStr, path::Path, io::BufReader};

use chrono::{TimeZone, Utc, DateTime};
use ring::signature::{RsaKeyPair, self};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ureq::{Agent, AgentBuilder};

use crate::authentication::{sign, AuthRequest, TokenResponse};
use crate::url::Url;
use crate::{Error, Result};

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/"; 
const TRANSIP_CONFIG_DIR: &str = "/home/paul/.config/transip";
const TOKEN_EXPIRATION_TIME: TokenExpiration = TokenExpiration::Hours(2);
const AGENT_TIMEOUT_SECONDS: u64 = 30;

pub trait Persist<T> where T: Serialize + DeserializeOwned {
    fn load(&self) -> Result<T>;
    fn dump(&self, t: T) -> Result<()>;
}

impl<T> Persist<T> for PathBuf where T: Serialize + DeserializeOwned {
    fn dump(&self, t: T) -> Result<()> {
        let file = std::fs::File::create(self)?;
        ureq::serde_json::to_writer_pretty(file, &t)?;
        Ok(())
    }

    fn load(&self) -> Result<T> {
        let file = std::fs::File::open(self)?;
        let t: T = ureq::serde_json::from_reader(file)?;
        Ok(t)
    }
}

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
        write!(f, "{}", match self {
            TokenExpiration::Seconds(seconds) => fmt(seconds, "second"),
            TokenExpiration::Minutes(minutes) => fmt(minutes, "minute"),
            TokenExpiration::Hours(hours) => fmt(hours, "hour"),
        })
    }
}


#[derive(Deserialize, Serialize)]
struct TokenResponseMeta {
    pub iss: String,
    pub aud: String,
    pub jti: String,
    pub iat: u64,
    pub nbf: u64,
    pub exp: u64,
    pub cid: String,
    pub ro: bool,
    pub gk: bool,
    pub kv: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub raw: String,
    pub expire: DateTime<Utc>,
}

impl Token {
    pub fn expired(&self) -> bool {
        (self.expire - Utc::now()).num_seconds() < 2
    }
}

pub struct AuthConfiguration {
    pub account_name: String,
    pub key: RsaKeyPair,
}

pub fn read_rsa_key_pair_from_pem<P>(path: P) -> Result<RsaKeyPair> 
where P: AsRef<Path>
{
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut buf_reader)?;
    if keys.is_empty() {
        Err(Error::Key("None"))
    }
    else {
        signature::RsaKeyPair::from_pkcs8(keys[0].as_slice())
        .map_err(|_| Error::Key("Invalid"))
    }
}

pub fn get_default_account() -> Result<AuthConfiguration> {
    read_dir(TRANSIP_CONFIG_DIR)?
    .filter_map(|de| de.ok())
    .map(|de| de.path())
    .find(|path| path.extension() == Some(OsStr::new("pem")))
    .and_then(|path| {
        read_rsa_key_pair_from_pem(&path).ok().map(|key| AuthConfiguration {
            account_name: path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
            key,
        })
    })
    .ok_or(Error::Key("No key for account"))
}

pub struct ApiClient {
    pub url: Url,
    pub auth_config: AuthConfiguration,
    pub token: Option<Token>,
    pub agent: Agent,
}

impl From<AuthConfiguration> for ApiClient {
    fn from(auth_config: AuthConfiguration) -> Self {
        let url = Url::new(TRANSIP_API_PREFIX.to_owned());
        let agent = AgentBuilder::new().timeout(Duration::from_secs(AGENT_TIMEOUT_SECONDS)).build();
        let token_file: PathBuf = [TRANSIP_CONFIG_DIR, "token.json"].iter().collect();
        let token: Option<Token> = token_file.load().ok();
        Self {
            url,
            auth_config,
            token,
            agent,
        }        
    }
}

impl Drop for ApiClient {
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            let token_file: PathBuf = [TRANSIP_CONFIG_DIR, "token.json"].iter().collect();
            if let Err(e) = token_file.dump(token) {
                tracing::error!("Failed to dump token to file: {}", e);
            }
        }
    }
}

impl ApiClient {
    fn refresh_token_if_needed(&mut self) -> Result<()> {
        if self.token.as_ref().map(|t| t.expired()) != Some(false) {
            let auth_request = AuthRequest::new(&self.auth_config.account_name, &TOKEN_EXPIRATION_TIME.to_string());
            let json = auth_request.json();
            let signature = sign(json.as_slice(), &self.auth_config.key)?;
            tracing::info!("Json signing success");
            let token_response = 
                self.agent.post(&self.url.auth())
                .set("Signature", &signature)
                .send_bytes(json.as_slice())
                .map_err(Box::new)?
                .into_json::<TokenResponse>()?;
            tracing::info!("Received token");
            let mut splitted = token_response.token.split('.');
            if splitted.clone().count() != 3 {
                Err(Error::Token)
            }
            else {
                let input = splitted.nth(1).ok_or(Error::Token)?;
                let decoded = base64::decode_config(input, base64::URL_SAFE)?;
                let s = from_utf8(decoded.as_slice())?;
                let token_meta = ureq::serde_json::from_str::<TokenResponseMeta>(s)?;
                let token = Token {
                    raw: token_response.token,
                    expire: Utc.timestamp_millis((token_meta.exp as i64) * 1000),
                };
                self.token = Some(token);
                Ok(())
            }    
        }
        else {
            Ok(())
        }
    }

    pub(crate) fn get<T>(&mut self, url: &str) -> Result<T>
    where T: DeserializeOwned
    {
        self.refresh_token_if_needed()?;
        let token = self.token.as_ref().ok_or(Error::Token)?;
        let json = self.agent.get(url)
        .set("Authorization", &format!("Bearer {}", token.raw))
        .call()
        .map_err(Box::new)?
        .into_json::<T>()?;
        Ok(json)
    }

    pub(crate) fn delete<T>(&mut self, url: &str, t: T) -> Result<()>
    where T: Serialize
    {
        self.refresh_token_if_needed()?;
        let token = self.token.as_ref().ok_or(Error::Token)?;
        self.agent.delete(url)
        .set("Authorization", &format!("Bearer {}", token.raw))
        .send_json(t)
        .map_err(Box::new)?;
        Ok(())
    }
    
    pub(crate) fn post<T>(&mut self, url: &str, t: T) -> Result<()>
    where T: Serialize
    {
        self.refresh_token_if_needed()?;
        let token = self.token.as_ref().ok_or(Error::Token)?;
        self.agent.post(url)
        .set("Authorization", &format!("Bearer {}", token.raw))
        .send_json(t)
        .map_err(Box::new)?;
        Ok(())
    }
}
