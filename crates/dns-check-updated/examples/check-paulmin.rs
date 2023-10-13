use std::time::Instant;

use dns_check_updated::has_acme_challenge;

// const NAMESERVERS: [&str; 3] = ["ns0.transip.net", "ns1.transip.nl", "ns2.transip.eu"];
const DOMAIN_NAME: &str = "paulmin.nl.";
// const ACME_CHALLENGE: &str = "_acme-challenge";
const CHALLENGE: &str = "JaJaNeeNee";

fn main() {
    let start = Instant::now();
    tracing_subscriber::fmt().init();
    tracing::info!("Checking for acme challenge {} in domain {}", CHALLENGE, DOMAIN_NAME);
    match has_acme_challenge(
        DOMAIN_NAME.into(),
        CHALLENGE.into(),
    ) {
        Ok(_) => {
            tracing::info!("Checking took {} seconds", start.elapsed().as_secs());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
