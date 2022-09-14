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
    client.nameserver_list(DOMAIN_NAME)?.trace();

    let is_acme_challenge = |entry: &DnsEntry| entry.name == *"_acme-challenge" && entry.entry_type == *"TXT";
    for dns_entry in client.dns_entry_list(DOMAIN_NAME)?.into_iter().filter(is_acme_challenge) {
        tracing::info!("Acme challenge found in domain {} with content {}", DOMAIN_NAME, dns_entry.content);
        client.dns_entry_delete(DOMAIN_NAME, dns_entry.clone())?;
        tracing::info!("{:10} {} = {} deleted", &dns_entry.entry_type, &dns_entry.name, &dns_entry.content);
    }

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