use std::str::from_utf8;

use domain::DnsEntry;
use error::Error;
use serde::{de::DeserializeOwned, Serialize};

use crate::domain::DnsEntryItem;

mod account;
mod authentication;
mod configuration;
mod dns;
mod domain;
mod error;
mod general;
mod requester;
mod token;
mod vps;

type Result<T> = std::result::Result<T, Error>;

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/"; 
const TRANSIP_PEM_FILENAME: &str = "/home/paul/.config/transip/desktop.pem";
const TRANSIP_USERNAME: &str = "paulusminus";
const EXPIRATION_TIME: &str = "2 minutes";
const DOMAIN_NAME: &str = "paulmin.nl";

fn is_acme_challenge(dns_entry: &DnsEntry) -> bool {
    dns_entry.entry_type.as_str() == "TXT" && dns_entry.name.as_str() == "_acme-challenge"
}

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

    let mut splitted = token_response.token.split(".");
    if splitted.clone().count() != 3 {
        return Err(Error::Token)
    }
    else {
        let input = splitted.nth(1).ok_or(Error::Token)?;
        let decoded = base64::decode_config(input, base64::URL_SAFE)?;
        let s = from_utf8(decoded.as_slice())?;
        let token = ureq::serde_json::from_str::<token::Token>(s)?;
        tracing::info!("Token valid for {} seconds", token.exp - token.nbf);
    }

    Ok::<String, Error>(token_response.token)
}

fn get<T>(token: &str, url: &str) -> Result<T>
where T: DeserializeOwned
{
    let json = ureq::get(url)
    .set("Authorization", &format!("Bearer {}", token))
    .call()?
    .into_json::<T>()?;
    Ok(json)
}

fn delete<T>(token: &str, url: &str, t: T) -> Result<()>
where T: Serialize
{
    ureq::delete(url)
    .set("Authorization", &format!("Bearer {}", token))
    .send_json(t)?;
    Ok(())
}

fn post<T>(token: &str, url: &str, t: T) -> Result<()>
where T: Serialize
{
    ureq::post(url)
    .set("Authorization", &format!("Bearer {}", token))
    .send_json(t)?;
    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let url = requester::Url::new(TRANSIP_API_PREFIX);
    let token = get_token(TRANSIP_USERNAME, EXPIRATION_TIME, TRANSIP_PEM_FILENAME)?;

    let ping = get::<general::Ping>(&token, &url.api_test())?;
    tracing::info!("Ping response: {}", ping.ping);

    // let vps_list = get::<vps::VpsList>(&token, &url.vps())?;
    // for vps in vps_list.vpss {
    //     tracing::info!("Vps: {} ({})", vps.name, vps.operating_system);
    // }

    // let invoice_list = get::<account::InvoiceList>(&token, &url.invoices())?;
    // for invoice in invoice_list.invoices {
    //     tracing::info!("Invoice: {}", invoice.invoice_number);
    // }

    // let nameserver_list = get::<domain::NameServerList>(&token, &url.domain_nameservers(DOMAIN_NAME))?;
    // for nameserver in nameserver_list.nameservers {
    //     tracing::info!("{}", nameserver.hostname);
    // }

    // let dns_entry_list = get::<domain::DnsEntryList>(&token, &url.domain_dns(DOMAIN_NAME))?;

    // for dns_entry in dns_entry_list.dns_entries.into_iter().filter(is_acme_challenge) {
    //     // tracing::info!("Acme challenge found in domain {} with content {}", DOMAIN_NAME, dns_entry.content);
    //     delete(&token, &url.domain_dns(DOMAIN_NAME), &domain::DnsEntryItem { dns_entry: dns_entry.clone() })?;
    //     tracing::info!("{:10} {} = {} deleted", dns_entry.entry_type, dns_entry.name, dns_entry.content);
    // }

    // let dns_entry = DnsEntryItem { dns_entry: DnsEntry { name: "_acme-challenge".into(), expire: 60, entry_type: "TXT".into(), content: "test".into() }};
    // post(&token, &url.domain_dns(DOMAIN_NAME), dns_entry)?;

    Ok(())
}
