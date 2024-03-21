use std::{
    ffi::OsString,
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Serialize;

use crate::client::Url;

pub use key_pair::KeyPair;
pub use token::{Token, TokenExpired, TokenResponse};
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

pub struct AuthRequestBuilder {
    login: String,
    nonce: String,
    read_only: bool,
    label: String,
    global_key: bool,
}

impl AuthRequestBuilder {
    pub fn new<S: Into<String>>(username: S) -> Self {
        Self {
            login: username.into(),
            nonce: milliseconds_since_epoch(),
            read_only: false,
            label: create_label(),
            global_key: true,
        }
    }

    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    pub fn whitelisted_only(mut self) -> Self {
        self.global_key = false;
        self
    }

    pub fn expiration(mut self, expiration: TokenExpiration) -> Self {
        self
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
    pub fn new(
        username: &str,
        expiration_time: TokenExpiration,
        read_only: bool,
        whitelisted_only: bool,
    ) -> Self {
        Self {
            login: username.into(),
            nonce: milliseconds_since_epoch(),
            read_only,
            expiration_time: expiration_time.to_string(),
            label: create_label(),
            global_key: !whitelisted_only,
        }
    }

    pub fn json(&self) -> Vec<u8> {
        ureq::serde_json::to_vec(self).unwrap()
    }
}

fn hostname_timestamp(hostname: OsString) -> String {
    format!(
        "{}-{}",
        hostname.to_string_lossy(),
        chrono::offset::Local::now().format("%Y%m%dT%H%M%S"),
    )
}

fn create_label() -> String {
    hostname::get().map(hostname_timestamp).unwrap_or(format!(
        "{} {}",
        env!("CARGO_PKG_NAME"),
        milliseconds_since_epoch(),
    ))
}

fn milliseconds_since_epoch() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}
