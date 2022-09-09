use std::{vec, path::Path, time::{SystemTime, UNIX_EPOCH}};
use ring::{self, signature::{self, RsaKeyPair}, rand};
use serde::Serialize;
use error::Error;

mod error;

type Result<T> = std::result::Result<T, Error>;

const TRANSIP_KEY_FILE: &str = "/home/paul/.config/transip/paulusminus.pk8";
const TRANSIP_USERNAME: &str = "paulusminus";
const MESSAGE: &[u8] = b"Hello, world, here I am";
const EXPIRATION_SECONDS: u16 = 60u16 * 60u16;

fn expiration_time() -> String {
    format!("{} seconds", EXPIRATION_SECONDS)
}

fn new_nonce() -> String {
    let nonce = 
        SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{:x}", nonce)
}

fn read_rsa_key_pair<P>(path: P) -> Result<RsaKeyPair> 
where P: AsRef<Path>
{
    let bytes = std::fs::read(path)?;
    RsaKeyPair::from_pkcs8(bytes.as_slice())
    .map_err(|e| Error::Key(e.description_().to_owned()))
}

fn sign(body: &[u8], key_pair: RsaKeyPair) -> Result<String> {
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

#[derive(Serialize)]
struct AuthRequest {
    login: String,
    nonce: String,
    label: Option<String>,
    read_only: bool,
    expiration_time: String,
    global_key: bool,
}

fn current_exe_name() -> Option<String> {
    std::env::current_exe()
    .ok()
    .and_then(
        |p|
            p
            .file_name()
            .map(|o| o.to_string_lossy().to_string())
    )
}

impl Default for AuthRequest {
    fn default() -> Self {
        let now = chrono::offset::Local::now().format("%F %T");
        Self {
            login: TRANSIP_USERNAME.to_owned(),
            nonce: new_nonce(),
            label: current_exe_name().map(|exe| format!("{} {}", exe, now)),
            read_only: false,
            expiration_time: expiration_time(),
            global_key: true,
        }
    }
}

fn main() -> Result<()> {
    let key_pair = read_rsa_key_pair(TRANSIP_KEY_FILE)?;

    println!("Nonce: {}", new_nonce());
    println!("Nonce: {}", new_nonce());

    let result = sign(MESSAGE, key_pair)?;
    print!("{}", result);
    Ok(())
}
