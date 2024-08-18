use std::error::Error;

use http_client::{Client, JsonApi};
use serde::{Deserialize, Serialize};

const IPIFY_IPV4_URL: &str = "https://api.ipify.org?format=json";
const IPIFY_IPV6_URL: &str = "https://api64.ipify.org?format=json";

#[derive(Debug, Deserialize, Serialize)]
struct Ipify {
    ip: String,
}

impl std::fmt::Display for Ipify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ip)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::default();

    let ipv4 = client.get::<Ipify>(IPIFY_IPV4_URL)?;
    let ipv6 = client.get::<Ipify>(IPIFY_IPV6_URL)?;
    println!("ipv4: {ipv4}, ip6: {ipv6}");

    Ok(())
}