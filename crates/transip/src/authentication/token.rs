use crate::{error::ResultExt, Error, Result};
use std::{
    io::{Cursor, Read, Write},
    path::Path,
};

use crate::base64::Base64;
use crate::fs::FileSystem;
use chrono::Utc;
use serde::{Deserialize, Serialize};

pub const DEMO_TOKEN: &str = include_str!("demo_token.txt");

pub trait TokenExpired {
    fn token_expired(&self) -> bool;
}

pub struct Token {
    raw: String,
    expired: i64,
}

impl Token {
    pub fn demo() -> Self {
        Self {
            raw: DEMO_TOKEN.to_owned(),
            expired: i64::MAX,
        }
    }

    pub fn try_from_reader<R>(mut reader: R) -> Result<Self>
    where
        R: Read,
    {
        let mut s = String::default();
        reader.read_to_string(&mut s)?;
        Token::try_from(s)
    }

    pub fn try_to_write<W>(&self, mut writer: W) -> Result<()>
    where
        W: Write,
    {
        writer.write_all(self.raw.as_bytes()).err_into()
    }

    pub fn try_to_write_file<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        path.writer().and_then(|file| self.try_to_write(file))
    }

    pub fn try_from_file<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        path.reader().and_then(Token::try_from_reader)
    }

    pub fn raw(&self) -> &str {
        self.raw.as_str()
    }
}

impl TryFrom<String> for Token {
    type Error = Error;
    fn try_from(raw: String) -> Result<Self> {
        token_expiration_timestamp(raw.clone()).map(|expired| Token { raw, expired })
    }
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

impl TryFrom<EncodedTokenMeta<'_>> for TokenResponseMeta {
    type Error = Error;
    fn try_from(encoded_token_meta: EncodedTokenMeta) -> Result<Self> {
        encoded_token_meta
            .expiration()
            .base64_decode_url_safe()
            .map(Cursor::new)
            .and_then(|cursor| ureq::serde_json::from_reader(cursor).err_into())
    }
}

struct EncodedTokenMeta<'a>(&'a str);

impl EncodedTokenMeta<'_> {
    pub fn expiration(&self) -> String {
        self.0.to_owned()
    }
}

impl TryFrom<&str> for TokenResponseMeta {
    type Error = Error;
    fn try_from(token: &str) -> Result<Self> {
        EncodedTokenMeta::try_from(token).and_then(TokenResponseMeta::try_from)
    }
}

impl<'a> TryFrom<&'a str> for EncodedTokenMeta<'a> {
    type Error = Error;
    fn try_from(token: &'a str) -> Result<Self> {
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
    // use super::EncodedTokenMeta;
    use super::{Token, TokenExpired};
    use chrono::Utc;

    // const RAW_TOKEN: &str = include_str!("/home/paul/transip/expired_token.txt");
    // const TOKEN_META_JSON: &str = include_str!("/home/paul/transip/token_meta.json");

    // fn expired_token() -> Result<String> {
    //     Ok(
    //         std::env::var("EXPIRED_TOKEN").unwrap_or(std::fs::read_to_string(
    //             "/home/paul/transip/expired_token.txt",
    //         )?),
    //     )
    // }

    // fn expired_token_meta_json() -> Result<String> {
    //     Ok(
    //         std::env::var("EXPIRED_TOKEN_META_JSON").unwrap_or(std::fs::read_to_string(
    //             "/home/paul/transip/token_meta.json",
    //         )?),
    //     )
    // }

    // #[test]
    // fn encoded_token_meta_try_from() {
    //     let token = expired_token().unwrap();
    //     let encoded = EncodedTokenMeta::try_from(token.as_str());
    //     assert!(encoded.is_ok());
    //     assert_eq!(
    //         encoded.unwrap().expiration(),
    //         "eyJpc3MiOiJhcGkudHJhbnNpcC5ubCIsImF1ZCI6ImFwaS50cmFuc2lwLm5sIiwianRpIjoiI3UlMnI0cmwlbz9Za1I2cHRITnUiLCJpYXQiOjE2OTY5MTQ0MzAsIm5iZiI6MTY5NjkxNDQzMCwiZXhwIjoxNjk2OTIxNjMwLCJjaWQiOjEwMTkxNCwicm8iOmZhbHNlLCJnayI6ZmFsc2UsImt2Ijp0cnVlfQ",
    //     );
    // }

    // #[test]
    // fn decode() {
    //     let token = expired_token().unwrap();
    //     let encoded_metadata = EncodedTokenMeta::try_from(token.as_str()).unwrap();
    //     let decoded = encoded_metadata.expiration().base64_decode_url_safe();
    //     assert!(decoded.is_ok());
    //     let token_meta = decoded.unwrap();
    //     let s = from_utf8(token_meta.as_slice()).unwrap();
    //     assert_eq!(s, expired_token_meta_json().unwrap());
    // }

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

    #[test]
    fn try_token_from_not_existing_file() {
        let result = Token::try_from_file("asdlkjfie3847");
        assert!(result.is_err());
    }

    #[test]
    fn try_token_from_existing_file() {
        if std::env::var("EXPIRED_TOKEN").is_err() {
            let filename = "/home/paul/transip/expired_token.txt";
            let result = Token::try_from_file(filename);
            assert!(result.is_ok());
            assert!(result.unwrap().token_expired());
        }
    }

    #[test]
    fn try_demo_token() {
        assert!(!Token::demo().token_expired());
    }
}
