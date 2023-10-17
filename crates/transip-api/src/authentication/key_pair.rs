use ring::{
    rand,
    signature::{self, RsaKeyPair},
};
use std::{
    fs::OpenOptions,
    io::{BufReader, Read},
    path::Path,
};

use crate::base64::Base64;
use crate::{Error, Result};

pub struct KeyPair {
    inner: RsaKeyPair,
}

impl From<RsaKeyPair> for KeyPair {
    fn from(rsa_key_pair: RsaKeyPair) -> Self {
        Self {
            inner: rsa_key_pair,
        }
    }
}

impl KeyPair {
    pub fn try_from_reader<R>(r: R) -> Result<KeyPair>
    where
        R: Read,
    {
        let keys = rustls_pemfile::pkcs8_private_keys(&mut BufReader::new(r))?;
        if keys.is_empty() {
            Err(Error::Key("None"))
        } else {
            signature::RsaKeyPair::from_pkcs8(keys[0].as_slice())
                .map_err(|error| Error::Rejected(error.to_string()))
                .map(KeyPair::from)
        }
    }

    pub fn try_from_file<P>(path: P) -> Result<KeyPair>
    where
        P: AsRef<Path>,
    {
        OpenOptions::new()
            .read(true)
            .open(path)
            .map_err(Into::into)
            .and_then(KeyPair::try_from_reader)
    }

    pub fn sign<S>(&self, data: S) -> Result<String>
    where
        S: AsRef<[u8]>,
    {
        let rng = rand::SystemRandom::new();
        let mut signature = vec![0; self.inner.public().modulus_len()];
        self.inner
            .sign(
                &signature::RSA_PKCS1_SHA512,
                &rng,
                data.as_ref(),
                &mut signature,
            )
            .map_err(Error::Sign)?;

        Ok(signature.as_slice().base64_encode_standard_padding())
    }
}

#[cfg(test)]
mod test {
    use super::KeyPair;

    #[test]
    fn sign() {
        let key_pair = KeyPair::try_from_file("/etc/transip/paulusminus.pem").unwrap();
        let signature = key_pair.sign("{}");
        assert!(signature.is_ok());
    }
}
