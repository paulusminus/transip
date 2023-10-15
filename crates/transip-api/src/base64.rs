use crate::Error;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;

pub trait Base64 {
    fn base64_decode(&self) -> Result<Vec<u8>, Error>;
    fn base64_encode(&self) -> String;
}

impl<T> Base64 for T
where
    T: AsRef<[u8]>,
{
    fn base64_decode(&self) -> Result<Vec<u8>, Error> {
        URL_SAFE_NO_PAD.decode(self).map_err(Error::from)
    }
    fn base64_encode(&self) -> String {
        URL_SAFE_NO_PAD.encode(self)
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;

    use crate::base64::Base64;

    const TEST_STRING: &str = "Hallo allemaal wat fijn dat u er bent";
    const TEST_STRING_ENCODED: &str = "SGFsbG8gYWxsZW1hYWwgd2F0IGZpam4gZGF0IHUgZXIgYmVudA";

    #[test]
    fn encode() {
        assert_eq!(TEST_STRING.base64_encode(), TEST_STRING_ENCODED,);
    }

    #[test]
    fn decode() {
        assert_eq!(
            from_utf8(TEST_STRING_ENCODED.base64_decode().unwrap().as_slice()).unwrap(),
            TEST_STRING,
        )
    }
}
