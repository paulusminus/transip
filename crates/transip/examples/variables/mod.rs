use std::env::set_var;

pub fn set_variables() {
    set_var("TRANSIP_API_USERNAME", "paulusminus");
    set_var("TRANSIP_API_PRIVATE_KEY", "/etc/transip/home.pem");
    set_var("TRANSIP_API_TOKEN_PATH", "/home/paul/.transip-token.txt");
    set_var("TRANSIP_API_READONLY", "false");
    set_var("TRANSIP_API_WHITELISTED_ONLY", "true");
    set_var("TRANSIP_API_TOKEN_EXPIRATION", "5 minutes");
}