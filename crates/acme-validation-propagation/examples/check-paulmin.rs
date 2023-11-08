use std::time::Instant;

use acme_validation_propagation::wait;

// const NAMESERVERS: [&str; 3] = ["ns0.transip.net", "ns1.transip.nl", "ns2.transip.eu"];
const DOMAIN_NAME: &str = "paulmin.nl.";
// const ACME_CHALLENGE: &str = "_acme-challenge";
const CHALLENGE: &str = "JaJaNeeNee";

fn main() {
    let start = Instant::now();
    tracing_subscriber::fmt().init();
    tracing::info!(
        "Checking for acme challenge {} in domain {}",
        CHALLENGE,
        DOMAIN_NAME
    );
    match wait(DOMAIN_NAME, CHALLENGE) {
        Ok(_) => {
            tracing::info!("Checking took {} seconds", start.elapsed().as_secs());
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
