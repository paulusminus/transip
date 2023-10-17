use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::api_client::Url;

pub use key_pair::KeyPair;
pub use token::{token_expiration_timestamp, Token, TokenExpired, TokenResponse};
pub use token_expiration::TokenExpiration;

mod key_pair;
mod token;
mod token_expiration;

const AUTH: &str = "auth";

pub trait UrlAuthentication {
    fn auth(&self) -> String;
}

impl UrlAuthentication for Url {
    fn auth(&self) -> String {
        format!("{}{}", self.prefix, AUTH)
    }
}

#[derive(Serialize)]
pub struct AuthRequest {
    login: String,
    nonce: String,
    read_only: bool,
    expiration_time: String,
    label: String,
    global_key: bool,
}

impl AuthRequest {
    pub fn new(username: &str, expiration_time: &str) -> Self {
        let now = chrono::offset::Local::now().format("%Y%m%dT%H%M");
        Self {
            login: username.into(),
            nonce: milliseconds_since_epoch(),
            read_only: false,
            expiration_time: expiration_time.into(),
            label: hostname::get()
                .map(|hostname| format!("{}-{}", hostname.to_string_lossy(), now))
                .unwrap_or_default(),
            global_key: true,
        }
    }

    pub fn json(&self) -> Vec<u8> {
        ureq::serde_json::to_vec(self).unwrap()
    }
}

fn milliseconds_since_epoch() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
