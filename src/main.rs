use std::{vec, path::Path, time::{SystemTime, UNIX_EPOCH}, fs::File, io::BufReader};
use ring::{self, signature::{self, RsaKeyPair}, rand};
use serde::Serialize;
use error::Error;

mod error;

type Result<T> = std::result::Result<T, Error>;

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/"; 
// const TRANSIP_KEY_FILE: &str = "/home/paul/.config/transip/paulusminus.pk8";
const TRANSIP_PEM_FILE: &str = "/home/paul/.config/transip/desktop.pem";
const TRANSIP_USERNAME: &str = "paulusminus";
const EXPIRATION_TIME: &str = "30 minutes";

fn transip_url(command: &str) -> String {
    format!("{TRANSIP_API_PREFIX}{command}")
}

fn new_nonce() -> String {
    SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis()
    .to_string()
}

fn read_rsa_key_pair_from_pem<P>(path: P) -> Result<RsaKeyPair> 
where P: AsRef<Path>
{
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut buf_reader)?;
    if keys.len() < 1 {
        Err(Error::Key("None".to_owned()))
    }
    else {
        RsaKeyPair::from_pkcs8(keys[0].as_slice())
        .map_err(|_| Error::Key("Invalid".to_owned()))
    }
}


// fn read_rsa_key_pair<P>(path: P) -> Result<RsaKeyPair> 
// where P: AsRef<Path>
// {
//     let bytes = std::fs::read(path)?;
//     RsaKeyPair::from_pkcs8(bytes.as_slice())
//     .map_err(|e| Error::Key(e.description_().to_owned()))
// }

fn sign(body: &[u8], key_pair: RsaKeyPair) -> Result<String> {
    let rng = rand::SystemRandom::new();
    let mut signature = vec![0; key_pair.public_modulus_len()];
    // let digest = ring::digest::digest(&ring::digest::SHA512, body);
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
    read_only: bool,
    expiration_time: String,
    label: String,
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
            read_only: false,
            expiration_time: EXPIRATION_TIME.to_owned(),
            label: current_exe_name().map(|exe| format!("{} {}", exe, now)).unwrap_or_default(),
            global_key: false,
        }
    }
}

fn main() -> Result<()> {
    let key_pair = read_rsa_key_pair_from_pem(TRANSIP_PEM_FILE)?;
    let auth_request = AuthRequest::default();
    let json = ureq::serde_json::to_vec(&auth_request)?;
    let json_string = ureq::serde_json::to_string_pretty(&auth_request)?;
    let signature = sign(json.as_slice(), key_pair)?;

    let auth_url = transip_url("auth");
    println!("Signature: {signature}");
    println!("Auth url: {auth_url}");
    println!("Json Body:\n{json_string}");

    {
        let response = 
            ureq::post(&auth_url)
            .set("Signature", &signature)
            .send_bytes(json.as_slice())?;

        print!("Response: {}", response.into_string()?);
    }

    Ok(())
}
