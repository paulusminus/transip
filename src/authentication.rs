use std::fs::{File};
use std::io::{BufReader};
use std::path::{Path};
use std::time::{SystemTime, UNIX_EPOCH};

use ring::{rand, signature};
use serde::{Serialize, Deserialize};

use crate::Result;
use crate::error::Error;

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

fn read_rsa_key_pair_from_pem<P>(path: P) -> Result<signature::RsaKeyPair> 
where P: AsRef<Path>
{
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut buf_reader)?;
    if keys.len() < 1 {
        Err(Error::Key("None".to_owned()))
    }
    else {
        signature::RsaKeyPair::from_pkcs8(keys[0].as_slice())
        .map_err(|_| Error::Key("Invalid".to_owned()))
    }
}

pub fn sign<P>(body: &[u8], path: P) -> Result<String> 
where P: AsRef<Path>
{
    let key_pair = read_rsa_key_pair_from_pem(path)?;
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
        base64::encode(signature.as_slice())
    )
}
