use dns_check_updated::servers_have_acme_challenge;

const NAMESERVERS: [&str; 3] = ["ns0.transip.net", "ns1.transip.nl", "ns2.transip.eu"];
const DOMAIN_NAME: &str = "paulmin.nl";
const ACME_CHALLENGE: &str = "_acme-challenge";
const CHALLENGE: &str = "JaJaNeeNee";

fn main() {
    match servers_have_acme_challenge(
        NAMESERVERS.iter(),
        DOMAIN_NAME,
        ACME_CHALLENGE,
        CHALLENGE.into(),
    ) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
