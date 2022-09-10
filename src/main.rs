use error::Error;
use serde::de::DeserializeOwned;

mod authentication;
mod domain;
mod error;
mod general;
mod vps;

type Result<T> = std::result::Result<T, Error>;

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/"; 
const TRANSIP_PEM_FILENAME: &str = "/home/paul/.config/transip/desktop.pem";
const TRANSIP_USERNAME: &str = "paulusminus";
const EXPIRATION_TIME: &str = "30 seconds";

fn transip_url(command: &str) -> String {
    format!("{TRANSIP_API_PREFIX}{command}")
}

fn get_token(username: &str, expiration_time: &str, pem_filename: &str) -> Result<String> {
    let auth_request = authentication::AuthRequest::new(username, expiration_time);
    let signature = authentication::sign(auth_request.json().as_slice(), pem_filename)?;
    let auth_url = transip_url("auth");
    let token_response = 
        ureq::post(&auth_url)
        .set("Signature", &signature)
        .send_bytes(auth_request.json().as_slice())?
        .into_json::<authentication::TokenResponse>()?;
    Ok::<String, Error>(token_response.token)
}

fn get<T>(token: &str, command: &str) -> Result<T>
where T: DeserializeOwned
{
    let json = ureq::get(&transip_url(command))
    .set("Authorization", &format!("Bearer {}", token))
    .call()?
    .into_json::<T>()?;
    Ok(json)
}

fn main() -> Result<()> {
    let token = get_token(TRANSIP_USERNAME, EXPIRATION_TIME, TRANSIP_PEM_FILENAME)?;

    let vpss = get::<vps::Vpss>(&token, "vps")?;
    println!("Total Vps: {}", vpss.vpss.len());
    for vps in vpss.vpss {
        println!("Vps: {} ({})", vps.name, vps.operating_system);
    }

    let invoices = get::<general::Invoices>(&token, "invoices")?;
    for invoice in invoices.invoices {
        println!("Invoice: {}", invoice.invoice_number);
    }

    let nameservers = get::<domain::NameServers>(&token, "domains/paulmin.nl/nameservers")?;
    for nameserver in nameservers.nameservers {
        println!("{}", nameserver.hostname);
    }

    let dns_entries = get::<domain::DnsEntries>(&token, "domains/paulmin.nl/dns")?;
    for dns_entry in dns_entries.dns_entries {
        println!("{:10} {} = {}", dns_entry.entry_type, dns_entry.name, dns_entry.content);
    }

    Ok(())
}
