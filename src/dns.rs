use std::{net::{SocketAddr, SocketAddrV4, Ipv4Addr}, thread::{sleep}, time::Duration};
use trust_dns_resolver::{Resolver, config::{ResolverConfig, NameServerConfig, Protocol, ResolverOpts}};

use crate::domain::NameServer;

use crate::error::Error;
use crate::Result;

fn is_ipv4(socket_address: &SocketAddr) -> bool {
    socket_address.is_ipv4()
}

fn to_resolver(nameserver: &NameServer) -> Result<Resolver> {
    let ipv_string = nameserver.ipv4.as_ref().ok_or(Error::Ipv4)?;
    let ipv4 = ipv_string.parse::<Ipv4Addr>()?;
    let socket_address = SocketAddr::V4(SocketAddrV4::new(ipv4, 53));
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

fn no_acme_challenge(resolver: &Resolver) -> bool {
    match resolver.txt_lookup("_acme-challenge.paulmin.nl.") {
        Ok(result) => result.as_lookup().records().to_vec().len() == 0,
        Err(_) => false,
    }
}

pub fn check_has_acme_challenge(nameservers: Vec<crate::domain::NameServer>) -> Result<()>{
    let resolvers = nameservers.iter().map(to_resolver).collect::<Result<Vec<Resolver>>>()?;
    let mut i = 0;
    while !resolvers.iter().all(has_acme_challenge) && i < 60 {
        i += 1;
        tracing::warn!("Attempt {} failed", i);
        sleep(Duration::from_secs(60));
    }

    if i >= 60 {
        tracing::error!("Failed to find acme challenge in {}", nameservers.into_iter().map(|n| n.hostname).collect::<Vec<String>>().join(", "));
        Err(Error::AcmeChallege)
    }
    else {
        Ok(())
    }

}