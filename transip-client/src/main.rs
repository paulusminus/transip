use std::net::IpAddr;

use transip_api::*;
use trace::VecExt;

const DOMAIN_NAME: &str = "paulmin.nl";

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut client: ApiClient = get_default_account()?.into();

    if client.api_test()?.as_str() != "pong" {
        return Err(Error::ApiTest);
    }

    client.availability_zones()?.trace();

    let products = client.products()?;
    products.vps.trace();
    products.haip.trace();
    products.private_networks.trace();
    client.product_elements("vps-bladevps-xs")?.trace();
    client.vps_list()?.trace();
    // client.invoice_list()?.trace();
    client.domain_list()?.trace();

    let ip_addresses = 
        client
        .nameserver_list(DOMAIN_NAME)?
        .iter()
        .map(|nameserver| nameserver.to_ip_address())
        .collect::<Result<Vec<IpAddr>>>()?;
    ip_addresses.trace();

    let is_acme_challenge = |entry: &DnsEntry| entry.name == *"_acme-challenge" && entry.entry_type == *"TXT";
    client.dns_entry_delete_all(DOMAIN_NAME, is_acme_challenge)?;

    // let dns_entry = DnsEntry { 
    //     name: "_acme-challenge".into(), 
    //     expire: 60,
    //     entry_type: "TXT".into(),
    //     content: "Testenmaar".into(), 
    // };
    // client.dns_entry_insert(DOMAIN_NAME, dns_entry)?;

    Ok(())
}

mod trace {
    use core::fmt::Display;

    pub trait VecExt {
        fn trace(&self);
    }
    
    impl<T> VecExt for Vec<T> where T: Display {
        fn trace(&self) {
            self.iter().for_each(trace_object)
        }
    }
    
    fn trace_object<T>(t: T) where T: Display {
        tracing::info!("{}", t)
    }    
}