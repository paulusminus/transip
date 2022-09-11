const NAMESERVERS: [&str; 3] = ["ns0.transip.net", "ns1.transip.nl", "ns2.transip.eu"];

use std::{net::{ToSocketAddrs, SocketAddr}, thread::{sleep}, time::Duration};
use trust_dns_resolver::{Resolver, config::{ResolverConfig, NameServerConfig, Protocol, ResolverOpts}};

pub type Error = Box<dyn std::error::Error>;

fn is_ipv4(socket_address: &SocketAddr) -> bool {
    socket_address.is_ipv4()
}

fn to_resolver(nameserver: &str) -> Result<Resolver, Error> {
    let socket_addresses = format!("{nameserver}:53").to_socket_addrs()?;
    let socket_address = socket_addresses.filter(is_ipv4).last().ok_or("no ipv4 address")?;
    let name_server_config = NameServerConfig::new(socket_address, Protocol::Udp);
    let mut resolver_config = ResolverConfig::new();
    resolver_config.add_name_server(name_server_config);
    let mut options = ResolverOpts::default();
    options.recursion_desired = false;
    options.use_hosts_file = false;
    Ok(Resolver::new(resolver_config, options)?)
}

fn has_acme_challenge(resolver: &Resolver) -> bool {
    match resolver.txt_lookup("_acme-challenge.paulmin.nl.") {
        Ok(result) => result.as_lookup().records().to_vec().len() > 0,
        Err(_) => false,
    }
}

fn main() -> Result<(), Error>{
    tracing_subscriber::fmt::init();
    let resolvers = NAMESERVERS.into_iter().map(to_resolver).collect::<Result<Vec<Resolver>, Error>>()?;
    let mut i = 0;
    while !resolvers.iter().all(has_acme_challenge) {
        i += 1;
        tracing::info!("Attempt {} failed", i);
        sleep(Duration::from_secs(60));
    }

    Ok(())
}