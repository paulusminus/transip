const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/"; 
const TRANSIP_PEM_FILENAME: &str = "/home/paul/.config/transip/desktop.pem";
const TRANSIP_USERNAME: &str = "paulusminus";
const TOKEN_EXPIRATION_TIME: &str = "30 seconds";
const DOMAIN_NAME: &str = "paulmin.nl";
const TIMEOUT_SECONDS: u16 = 30;

pub struct Configuration<'a> {
    pub prefix: &'a str,
    pub private_key_filename: &'a str,
    pub username: &'a str,
    pub domain_name: &'a str,
    pub token_expiration_time: &'a str,
    pub timeout: u16,
}

impl<'a> Default for Configuration<'a> {
    fn default() -> Self {
        Self {
            prefix: TRANSIP_API_PREFIX,
            private_key_filename: TRANSIP_PEM_FILENAME,
            username: TRANSIP_USERNAME,
            domain_name: DOMAIN_NAME,
            token_expiration_time: TOKEN_EXPIRATION_TIME,
            timeout: TIMEOUT_SECONDS,
        }
    }
}