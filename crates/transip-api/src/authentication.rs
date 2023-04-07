use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::STANDARD as base64;
use base64::{Engine};
use ring::signature::RsaKeyPair;
use ring::{rand, signature};
use serde::{Serialize, Deserialize};

use crate::Result;
use crate::api_client::Url;
use crate::error::Error;

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
            label: hostname::get().map(|hostname| format!("{}-{}", hostname.to_string_lossy(), now)).unwrap_or_default(),
            global_key: false,
        }
    }

    pub fn json(&self) -> Vec<u8> {
        ureq::serde_json::to_vec(self).unwrap()
    }
}

#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    pub token: String,
}

fn milliseconds_since_epoch() -> String {
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
    .to_string()
}


pub fn sign(body: &[u8], key_pair: &RsaKeyPair) -> Result<String> 
{
    let rng = rand::SystemRandom::new();
    let mut signature = vec![0; key_pair.public_modulus_len()];
    key_pair.sign(
        &signature::RSA_PKCS1_SHA512,
        &rng,
        body,
        &mut signature,
    )
    .map_err(Error::Sign)?;
    
    Ok(
        base64.encode(signature.as_slice())
    )
}
