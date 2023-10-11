use crate::{Error, Result};
use std::io::Cursor;

use base64::{engine, Engine};
use serde::{Deserialize, Serialize};

pub fn token_expiration_timestamp<S>(token: S) -> Result<i64>
where
    S: AsRef<str>,
{
    TokenResponseMeta::try_from(token.as_ref()).map(|token_meta| token_meta.exp)
}

#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize)]
struct TokenResponseMeta {
    pub iss: String,
    pub aud: String,
    pub jti: String,
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
    pub cid: i64,
    pub ro: bool,
    pub gk: bool,
    pub kv: bool,
}

impl<'a> TryFrom<EncodedTokenMeta<'a>> for TokenResponseMeta {
    type Error = Error;
    fn try_from(encoded_token_meta: EncodedTokenMeta) -> std::result::Result<Self, Self::Error> {
        engine::general_purpose::URL_SAFE_NO_PAD
            .decode(encoded_token_meta.expiration())
            .map_err(Error::from)
            .map(Cursor::new)
            .and_then(|cursor| ureq::serde_json::from_reader(cursor).map_err(Error::from))
    }
}

struct EncodedTokenMeta<'a>(&'a str);

impl<'a> EncodedTokenMeta<'a> {
    pub fn expiration(&self) -> String {
        self.0.to_owned()
    }
}

impl TryFrom<&str> for TokenResponseMeta {
    type Error = Error;
    fn try_from(token: &str) -> std::result::Result<Self, Self::Error> {
        EncodedTokenMeta::try_from(token).and_then(TokenResponseMeta::try_from)
    }
}

impl<'a> TryFrom<&'a str> for EncodedTokenMeta<'a> {
    type Error = Error;
    fn try_from(token: &'a str) -> std::result::Result<Self, Self::Error> {
        let splitted = token.split('.').collect::<Vec<&str>>();

        if splitted.len() == 3 {
            Ok(EncodedTokenMeta(splitted[1]))
        } else {
            Err(Error::Token)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EncodedTokenMeta;
    use base64::{engine::general_purpose, Engine};
    use std::str::from_utf8;

    const RAW_TOKEN: &str = include_str!("raw_token.txt");
    const TOKEN_META_JSON: &str = include_str!("token_meta.json");

    #[test]
    fn encoded_token_meta_try_from() {
        let encoded = EncodedTokenMeta::try_from(RAW_TOKEN);
        assert!(encoded.is_ok());
        assert_eq!(
            encoded.unwrap().expiration(),
            "eyJpc3MiOiJhcGkudHJhbnNpcC5ubCIsImF1ZCI6ImFwaS50cmFuc2lwLm5sIiwianRpIjoiI3UlMnI0cmwlbz9Za1I2cHRITnUiLCJpYXQiOjE2OTY5MTQ0MzAsIm5iZiI6MTY5NjkxNDQzMCwiZXhwIjoxNjk2OTIxNjMwLCJjaWQiOjEwMTkxNCwicm8iOmZhbHNlLCJnayI6ZmFsc2UsImt2Ijp0cnVlfQ"
        );
    }

    #[test]
    fn decode() {
        let encoded_metadata = EncodedTokenMeta::try_from(RAW_TOKEN).unwrap();
        let decoded = general_purpose::STANDARD_NO_PAD.decode(encoded_metadata.expiration());
        assert!(decoded.is_ok());
        let token_meta = decoded.unwrap();
        let s = from_utf8(token_meta.as_slice()).unwrap();
        assert_eq!(s, TOKEN_META_JSON);
    }
}
