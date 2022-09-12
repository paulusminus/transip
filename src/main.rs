use domain::{DnsEntry};
use error::Error;
use crate::configuration::{get_default_account, ApiClient, TransipApi};

mod account;
mod authentication;
mod configuration;
// mod dns;
mod domain;
mod error;
mod general;
mod url;
mod token;
mod vps;

type Result<T> = std::result::Result<T, Error>;

const DOMAIN_NAME: &str = "paulmin.nl";
const ACME_CHALLENGE: &str = "_acme-challenge";

fn is_acme_challenge(dns_entry: &DnsEntry) -> bool {
    dns_entry.entry_type.as_str() == "TXT" && dns_entry.name.as_str() == ACME_CHALLENGE
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut client: ApiClient = get_default_account()?.into();

    if client.api_test()?.as_str() != "pong" {
        return Err(Error::ApiTest);
    }

    for vps in client.vps_list()? {
        tracing::info!("Vps: {} ({})", vps.name, vps.operating_system);
    }

    for invoice in client.invoice_list()? {
        tracing::info!("Invoice: {}", invoice.invoice_number);
    }

    for nameserver in client.nameserver_list(DOMAIN_NAME)? {
        tracing::info!("{}", nameserver.hostname);
    }

    for dns_entry in client.dns_entry_list(DOMAIN_NAME)?.into_iter().filter(is_acme_challenge) {
        tracing::info!("Acme challenge found in domain {} with content {}", DOMAIN_NAME, dns_entry.content);
        client.dns_entry_delete(DOMAIN_NAME, dns_entry.clone().into())?;
        tracing::info!("{:10} {} = {} deleted", &dns_entry.entry_type, &dns_entry.name, &dns_entry.content);
    }

    let dns_entry = DnsEntry { 
        name: ACME_CHALLENGE.into(), 
        expire: 60,
        entry_type: "TXT".into(),
        content: "Testenmaar".into(), 
    };
    client.dns_entry_insert(DOMAIN_NAME, dns_entry.into())?;

    Ok(())
}
