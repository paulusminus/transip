use std::{net::{IpAddr}, thread::{sleep}, time::Duration};
use trust_dns_resolver::{Resolver, config::{ResolverConfig, ResolverOpts, NameServerConfigGroup}};

use crate::error::Error;

mod error;

pub type Result<T> = std::result::Result<T, Error>;

fn default_resolver() -> Result<Resolver> {
    Resolver::new(ResolverConfig::default(), ResolverOpts::default()).map_err(Error::from)
}

fn resolve_hostname<S>(hostname: S, resolver: &Resolver) -> Result<Vec<IpAddr>>
where S: AsRef<str>
{
    resolver.lookup_ip(hostname.as_ref())
    .map_err(Error::from)
    .map(|response| response.into_iter().collect())

}

fn to_resolver_by_resolver<S>(resolver: Resolver) -> impl Fn(S) -> Result<Resolver>
where S: AsRef<str>
{
    move |nameserver| {
        let ip_addresses = resolve_hostname(nameserver, &resolver)?;
        let nameserver_config_group = NameServerConfigGroup::from_ips_clear(&ip_addresses, 53, false);
        let resolver_config = ResolverConfig::from_parts(None, vec![], nameserver_config_group);
        let mut options = ResolverOpts::default();
        options.recursion_desired = false;
        options.use_hosts_file = false;
        Resolver::new(resolver_config, options).map_err(Error::from)
    }
}

fn has_acme_challenge_domain(record_name: &str) -> impl Fn(&Resolver) -> bool + '_ {
    move |resolver|
        match resolver.txt_lookup(record_name) {
            Ok(result) => {
                result.as_lookup().record_iter().for_each(|record| tracing::info!("{}", record));
                !result.as_lookup().records().to_vec().is_empty()
            },
            Err(_) => false,
        }
}

#[allow(dead_code)]
fn no_acme_challenge(resolver: &Resolver) -> bool {
    match resolver.txt_lookup("_acme-challenge.paulmin.nl.") {
        Ok(result) => {

            result.as_lookup().records().to_vec().is_empty()
        },
        Err(_) => false,
    }
}

pub fn servers_have_acme_challenge<I, S>(nameservers: I, domain_name: &str, acme_challenge: &str) -> Result<()> 
where I: Iterator<Item = S>, S: AsRef<str>
{
    let default_resolver = default_resolver()?;
    let resolvers = nameservers.map(to_resolver_by_resolver(default_resolver)).collect::<Result<Vec<Resolver>>>()?;
    let mut i = 0;
    while !resolvers.iter().all(has_acme_challenge_domain(&format!("{}.{}.", acme_challenge, domain_name))) && i < 60 {
        i += 1;
        tracing::warn!("Attempt {} failed", i);
        sleep(Duration::from_secs(60));
    }

    if i >= 60 {
        tracing::error!("Failed to find acme challenge");
        Err(Error::AcmeChallege)
    }
    else {
        Ok(())
    }
}