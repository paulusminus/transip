use crate::error::ResultExt;
use crate::Result;
use base64::engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD};
use base64::Engine;

pub trait Base64 {
    fn base64_decode_url_safe(&self) -> Result<Vec<u8>>;
    // fn base64_encode_url_safe(&self) -> String;
    fn base64_encode_standard_padding(&self) -> String;
}

impl<T> Base64 for T
where
    T: AsRef<[u8]>,
{
    fn base64_decode_url_safe(&self) -> Result<Vec<u8>> {
        URL_SAFE_NO_PAD.decode(self).err_into()
    }

    // fn base64_encode_url_safe(&self) -> String {
    //     URL_SAFE_NO_PAD.encode(self)
    // }

    fn base64_encode_standard_padding(&self) -> String {
        STANDARD.encode(self)
    }
}

#[cfg(test)]
mod test {
    use std::str::from_utf8;

    use crate::base64::Base64;

    const TEST_STRING: &str = "Hallo allemaal wat fijn dat u er bent";
    const TEST_STRING_ENCODED: &str = "SGFsbG8gYWxsZW1hYWwgd2F0IGZpam4gZGF0IHUgZXIgYmVudA";

    // #[test]
    // fn encode() {
    //     assert_eq!(TEST_STRING.base64_decode_url_safe(), TEST_STRING_ENCODED,);
    // }

    #[test]
    fn decode() {
        assert_eq!(
            from_utf8(
                TEST_STRING_ENCODED
                    .base64_decode_url_safe()
                    .unwrap()
                    .as_slice()
            )
            .unwrap(),
            TEST_STRING,
        )
    }
}
