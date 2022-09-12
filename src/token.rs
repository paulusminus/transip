use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize)]
pub struct TokenMeta {
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