use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Token {
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
