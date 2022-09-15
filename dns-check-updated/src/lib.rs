use std::{net::{SocketAddr}, thread::{sleep}, time::Duration};
use trust_dns_resolver::{Resolver, config::{ResolverConfig, NameServerConfig, Protocol, ResolverOpts}};
use crate::error::Error;

mod error;

pub type Result<T> = std::result::Result<T, Error>;

fn to_resolver(nameserver: std::net::IpAddr) -> Result<Resolver> {
    let socket_address =SocketAddr::new(nameserver, 53);
    let name_server_config = NameServerConfig::new(socket_address, Protocol::Udp);
    let mut resolver_config = ResolverConfig::new();
    resolver_config.add_name_server(name_server_config);
    let mut options = ResolverOpts::default();
    options.recursion_desired = false;
    options.use_hosts_file = false;
    Ok(Resolver::new(resolver_config, options)?)
}

fn has_acme_challenge_domain(domain_name: &str) -> impl Fn(&Resolver) -> bool + '_ {
    move |resolver|
        match resolver.txt_lookup(&format!("_acme_challenge.{}.", domain_name)) {
            Ok(result) => !result.as_lookup().records().to_vec().is_empty(),
            Err(_) => false,
        }
}

#[allow(dead_code)]
fn no_acme_challenge(resolver: &Resolver) -> bool {
    match resolver.txt_lookup("_acme-challenge.paulmin.nl.") {
        Ok(result) => result.as_lookup().records().to_vec().is_empty(),
        Err(_) => false,
    }
}

pub fn servers_have_acme_challenge(nameservers: impl Iterator<Item = std::net::IpAddr> + Copy, domain_name: &str) -> Result<()>{
    let resolvers = nameservers.map(to_resolver).collect::<Result<Vec<Resolver>>>()?;
    let mut i = 0;
    while !resolvers.iter().all(has_acme_challenge_domain(domain_name)) && i < 60 {
        i += 1;
        tracing::warn!("Attempt {} failed", i);
        sleep(Duration::from_secs(60));
    }

    if i >= 60 {
        tracing::error!("Failed to find acme challenge in {}", nameservers.map(|n| format!("{n}") ).collect::<Vec<String>>().join(", "));
        Err(Error::AcmeChallege)
    }
    else {
        Ok(())
    }
}