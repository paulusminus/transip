use core::fmt::Display;
use core::str::from_utf8;
use core::time::Duration;

use std::fs::{File, OpenOptions};
use std::io::{stdout, Stdout};
use std::path::PathBuf;
use std::{ffi::OsStr, fs::read_dir, io::BufReader, path::Path};

use base64::{engine, Engine};
use chrono::{DateTime, TimeZone, Utc};
use lazy_static::lazy_static;
use ring::signature::{self, RsaKeyPair};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ureq::{Agent, AgentBuilder};

use crate::authentication::{sign, AuthRequest, TokenResponse, UrlAuthentication};
use crate::{Error, Result};

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/";
const TOKEN_EXPIRATION_TIME: TokenExpiration = TokenExpiration::Hours(2);
const AGENT_TIMEOUT_SECONDS: u64 = 30;

lazy_static! {
    static ref TRANSIP_CONFIG_DIR: PathBuf = {
        std::env::var("TRANSIP_KEYS_DIR")
            .map(PathBuf::from)
            .ok()
            .or(directories::ProjectDirs::from("nl", "paulmin", "transip")
                .map(|proj_dir| proj_dir.config_dir().to_path_buf()))
            .unwrap()
    };
}

pub trait Persist<T>
where
    T: Serialize + DeserializeOwned,
{
    fn load(&self) -> Result<T>;
    fn dump(&self, t: T) -> Result<()>;
}

impl<T> Persist<T> for PathBuf
where
    T: Serialize + DeserializeOwned,
{
    fn dump(&self, t: T) -> Result<()> {
        std::fs::File::create(self)
            .map_err(Into::into)
            .and_then(|file| ureq::serde_json::to_writer_pretty(file, &t).map_err(Into::into))
    }

    fn load(&self) -> Result<T> {
        std::fs::File::open(self)
            .map_err(Into::into)
            .and_then(|file| ureq::serde_json::from_reader(file).map_err(Into::into))
    }
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
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut buf_reader)?;
    if keys.is_empty() {
        Err(Error::Key("None"))
    } else {
        signature::RsaKeyPair::from_pkcs8(keys[0].as_slice()).map_err(|_| Error::Key("Invalid"))
    }
}

pub fn default_account() -> Result<AuthConfiguration> {
    read_dir(TRANSIP_CONFIG_DIR.as_path())?
        .filter_map(|de| de.ok())
        .map(|de| de.path())
        .find(|path| path.extension() == Some(OsStr::new("pem")))
        .and_then(|path| {
            read_rsa_key_pair_from_pem(&path)
                .ok()
                .map(|key| AuthConfiguration {
                    account_name: path
                        .file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string(),
                    key,
                })
        })
        .ok_or(Error::Key("No key for account"))
}

pub struct Url {
    pub prefix: String,
}

impl Url {
    pub(crate) fn new(prefix: String) -> Self {
        Self { prefix }
    }
}

pub enum WriteWrapper {
    File(File),
    Stdout(Stdout),
}

impl std::io::Write for WriteWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::File(file) => file.write(buf),
            Self::Stdout(stdout) => stdout.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::File(file) => file.flush(),
            Self::Stdout(stdout) => stdout.flush(),
        }
    }
}

/// ApiClient is the main entry for this library. Creation is done by using ApiClient::new().
/// After creation of a client, this client can be used to call a Transip API call.
/// Each call starts with a check to see if we have a valid JWT token
/// If the token is expired or non existant then the Transip API call for requesting a new token is called
/// Tokens are persisted to disk on exit and reused if not expired on run
pub struct ApiClient {
    pub(crate) url: Url,
    auth_config: AuthConfiguration,
    token: Option<Token>,
    agent: Agent,
}

impl From<AuthConfiguration> for (ApiClient, WriteWrapper) {
    fn from(auth_config: AuthConfiguration) -> Self {
        let url = Url::new(TRANSIP_API_PREFIX.to_owned());
        let agent = AgentBuilder::new()
            .timeout(Duration::from_secs(AGENT_TIMEOUT_SECONDS))
            .build();
        let token_file: PathBuf = TRANSIP_CONFIG_DIR.join("token.json");
        let token: Option<Token> = token_file.load().ok();
        let log_file: PathBuf = TRANSIP_CONFIG_DIR.join("transip.log");

        let write_wrapper = match OpenOptions::new().append(true).open(log_file) {
            Ok(file) => WriteWrapper::File(file),
            Err(_) => WriteWrapper::Stdout(stdout()),
        };

        (
            ApiClient {
                url,
                auth_config,
                token,
                agent,
            },
            write_wrapper,
        )
    }
}

impl Drop for ApiClient {
    fn drop(&mut self) {
        if let Some(token) = self.token.take() {
            let token_file: PathBuf = TRANSIP_CONFIG_DIR.join("token.json");
            if let Err(e) = token_file.dump(token) {
                tracing::error!("Failed to dump token to file: {}", e);
            }
        }
    }
}

impl ApiClient {
    fn refresh_token_if_needed(&mut self) -> Result<()> {
        if self.token.as_ref().map(|t| t.expired()) != Some(false) {
            let auth_request = AuthRequest::new(
                &self.auth_config.account_name,
                &TOKEN_EXPIRATION_TIME.to_string(),
            );
            let json = auth_request.json();
            let signature = sign(json.as_slice(), &self.auth_config.key)?;
            tracing::info!("Json signing success");
            let token_response = self
                .agent
                .post(&self.url.auth())
                .set("Signature", &signature)
                .send_bytes(json.as_slice())
                .map_err(Box::new)?
                .into_json::<TokenResponse>()?;
            tracing::info!("Received token");
            let mut splitted = token_response.token.split('.');
            if splitted.clone().count() != 3 {
                Err(Error::Token)
            } else {
                let input = splitted.nth(1).ok_or(Error::Token)?;
                let decoded = engine::general_purpose::STANDARD.decode(input)?;
                let s = from_utf8(decoded.as_slice())?;
                let token_meta = ureq::serde_json::from_str::<TokenResponseMeta>(s)?;
                let token = Token {
                    raw: token_response.token,
                    expire: Utc
                        .timestamp_millis_opt((token_meta.exp as i64) * 1000)
                        .unwrap(),
                };
                self.token = Some(token);
                Ok(())
            }
        } else {
            Ok(())
        }
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
            .set("Authorization", &format!("Bearer {}", token.raw))
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
            .set("Authorization", &format!("Bearer {}", token.raw))
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
            .set("Authorization", &format!("Bearer {}", token.raw))
            .send_json(t)
            .map_err(Box::new)?;
        Ok(())
    }
}
