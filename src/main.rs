use error::Error;

mod error;
mod sign;
mod vps;

type Result<T> = std::result::Result<T, Error>;

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/"; 
const TRANSIP_PEM_FILE: &str = "/home/paul/.config/transip/desktop.pem";

fn transip_url(command: &str) -> String {
    format!("{TRANSIP_API_PREFIX}{command}")
}

fn main() -> Result<()> {

    let key_pair = sign::read_rsa_key_pair_from_pem(TRANSIP_PEM_FILE)?;
    let auth_request = sign::AuthRequest::default();
    let json = ureq::serde_json::to_vec(&auth_request)?;
    let json_string = ureq::serde_json::to_string_pretty(&auth_request)?;
    let signature = sign::sign(json.as_slice(), key_pair)?;

    println!("Signature: {signature}");
    println!("Json Body:\n{json_string}");

    let token = {
        let auth_url = transip_url("auth");
        let token_response = 
            ureq::post(&auth_url)
            .set("Signature", &signature)
            .send_bytes(json.as_slice())?
            .into_json::<sign::TokenResponse>()?;
        Ok::<String, Error>(token_response.token)
    }?;

    let vpss = 
        ureq::get(&transip_url("vps"))
        .set("Authorization", &format!("Bearer {}", token))
        .call()?
        .into_json::<vps::Vpss>()?;

    println!("Total Vps: {}", vpss.vpss.len());
    for vps in vpss.vpss {
        println!("Vps: {}", vps.name);
    }

    Ok(())
}
