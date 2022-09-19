use dns_check_updated::{servers_have_acme_challenge};

fn main() {
    let nameservers = vec![
        "ns0.transip.net".to_owned(),
        "ns1.transip.nl".to_owned(),
        "ns2.transip.eu".to_owned(),
    ].into_iter();
    match servers_have_acme_challenge(nameservers, "paulmin.nl", "_acme-challenge") {
        Ok(_) => {},
        Err(e) => { eprintln!("Error: {}", e); }
    }
}