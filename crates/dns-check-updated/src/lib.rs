use std::{net::IpAddr, thread::sleep, time::Duration, convert::identity};
use trust_dns_resolver::{
    config::{NameServerConfigGroup, ResolverConfig, ResolverOpts, LookupIpStrategy, GOOGLE_IPS},
    Resolver,
};

use crate::error::Error;

mod error;
mod resolver;

pub type Result<T> = std::result::Result<T, Error>;

fn google_nameservers_ipv6() -> Vec<IpAddr> {
    GOOGLE_IPS
    .into_iter()
    .filter(|ip| ip.is_ipv6())
    .cloned()
    .collect()
}

fn ipv6_resolver(group: NameServerConfigGroup, recursion: bool) -> Result<Resolver> {
    let config = ResolverConfig::from_parts(
        None,
        vec![],
        group,
    );
    let mut options = ResolverOpts::default();
    options.ip_strategy = LookupIpStrategy::Ipv6Only;
    options.recursion_desired = recursion;
    options.use_hosts_file = false;
    Resolver::new(config, options).map_err(Error::from)
}

fn google_resolver_ipv6_only() -> Result<Resolver> {
    let group = NameServerConfigGroup::from_ips_clear(
        &google_nameservers_ipv6().as_slice(),
        53,
        false,
    );
    ipv6_resolver(group, true)
}

fn default_resolver() -> Result<Resolver> {
    Resolver::from_system_conf().map_err(Error::from)
}

fn resolve_hostname<S>(hostname: S, resolver: &Resolver) -> Result<Vec<IpAddr>>
where
    S: AsRef<str>,
{
    resolver
        .lookup_ip(hostname.as_ref())
        .map_err(Error::from)
        .map(|response| response.into_iter().collect())
}

fn to_resolver_by_resolver<S>(resolver: Resolver) -> impl Fn(S) -> Result<Resolver>
where
    S: AsRef<str>,
{
    move |nameserver| {
        let ip_addresses = resolve_hostname(nameserver, &resolver)?;
        let nameserver_config_group =
            NameServerConfigGroup::from_ips_clear(&ip_addresses, 53, false);
        let resolver_config = ResolverConfig::from_parts(None, vec![], nameserver_config_group);
        let mut options = ResolverOpts::default();
        options.recursion_desired = false;
        options.use_hosts_file = false;
        Resolver::new(resolver_config, options).map_err(Error::from)
    }
}

fn has_acme_challenge_domain(record_name: &str, value: String) -> impl Fn(&Resolver) -> bool + '_ {
    move |resolver| match resolver.txt_lookup(record_name) {
        Ok(result) => {
            let answers = result.as_lookup().record_iter().filter(|r| {
                r.data().is_some()
                    && r.data().unwrap().as_txt().is_some()
                    && r.data().unwrap().as_txt().unwrap().to_string() == value
            });
            // answers.for_each(|record| tracing::info!("{}", record));
            answers.count() > 0
            // !result.as_lookup().records().to_vec().is_empty()
        }
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

pub fn has_acme_challenge(domain_name: String, challenge: String) -> Result<()> {
    let resolvers = {
        resolver::RecursiveIpv6Resolver::try_new()
            .and_then(|resolver| resolver.authoritive_ipv6_resolvers(domain_name.clone()))
    }?;

    let mut i: u128 = 0;

    while   !resolvers.iter()
                .map(|resolver| resolver.has_single_acme(domain_name.clone(), challenge.clone()))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .all(identity) 
            && i < 60
    {
        i += 1;
        tracing::warn!("Attempt {} failed", i);
        sleep(Duration::from_secs(60));
    }
    if i >= 60 {
        tracing::error!("Timeout checking acme challenge record");
        Err(Error::AcmeChallege)
    }
    else { Ok(()) }
}

pub fn servers_have_acme_challenge<I, S>(
    nameservers: I,
    domain_name: &str,
    acme_challenge: &str,
    challenge: &str,
) -> Result<()>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let default_resolver = default_resolver()?;
    let resolvers = nameservers
        .map(to_resolver_by_resolver(default_resolver))
        .collect::<Result<Vec<Resolver>>>()?;
    let mut i = 0;
    let record_name = format!("{}.{}.", acme_challenge, domain_name);
    while !resolvers
        .iter()
        .all(has_acme_challenge_domain(&record_name, challenge.into()))
        && i < 60
    {
        i += 1;
        tracing::warn!("Attempt {} failed", i);
        sleep(Duration::from_secs(60));
    }

    if i >= 60 {
        tracing::error!("1 hour timeout finding acme challenge");
        Err(Error::AcmeChallege)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Display, net::IpAddr};

    use trust_dns_resolver::{proto::rr::rdata::{AAAA, NS}, Resolver, lookup::{Ipv6Lookup, NsLookup}};

    use crate::{google_resolver_ipv6_only, error::Error};

    fn to_string<D: Display>(d: D) -> String { d.to_string() }

    fn aaaa_to_ipv6(aaaa: AAAA) -> IpAddr {
        IpAddr::V6((*aaaa).clone())
    }

    fn lookup(name: &str) -> impl Fn(Resolver) -> Result<Ipv6Lookup, Error> + '_ {
        move |resolver| resolver.ipv6_lookup(name).map_err(Error::from)
    }

    fn ns_lookup(name: &str) -> impl Fn(Resolver) -> Result<NsLookup, Error> + '_ {
        move |resolver| resolver.ns_lookup(name).map_err(Error::from)
    }

    fn aaaa_mapper(f: fn(AAAA) -> IpAddr) -> impl Fn(Ipv6Lookup) -> Vec<IpAddr> {
        move |lookup| lookup.into_iter().map(f).collect()
    }

    fn ns_mapper(f: fn(NS) -> String) -> impl Fn(NsLookup) -> Vec<String> {
        move |lookup| lookup.into_iter().map(f).collect()
    }

    fn ipv6_address_lookup(name: &str) -> Result<Vec<IpAddr>, Error> {
        google_resolver_ipv6_only()
            .and_then(lookup(name))
            .map(aaaa_mapper(aaaa_to_ipv6))
    }

    fn nameservers_lookup(name: &str) -> Result<Vec<String>, Error> {
        google_resolver_ipv6_only()
            .and_then(ns_lookup(name))
            .map(ns_mapper(to_string))
    }

    fn transip_resolvers_lookup(name: &str) -> Result<Vec<String>, Error> {
        google_resolver_ipv6_only()
            .and_then(ns_lookup(name))
            .map(ns_mapper(to_string))
    }


    #[test]
    fn test_www_paulmin_nl() {
        assert_eq!(
            ipv6_address_lookup("www.paulmin.nl.").unwrap(),
            vec![
                "2a01:7c8:bb0d:1bf:5054:ff:fedc:a36b".parse::<IpAddr>().unwrap(),
            ]
        );
    }

    #[test]
    fn test_ns0_transip_net() {
        assert_eq!(
            ipv6_address_lookup("ns0.transip.net").unwrap(),
            vec![
                "2a01:7c8:dddd:195::195".parse::<IpAddr>().unwrap(),
            ],
        );
    }

    #[test]
    fn test_ns1_transip_nl() {
        assert_eq!(
            ipv6_address_lookup("ns1.transip.nl.").unwrap(),
            vec![
                "2a01:7c8:7000:195::195".parse::<IpAddr>().unwrap(),
            ],
        );
    }

    #[test]
    fn test_ns2_transip_eu() {
        assert_eq!(
            ipv6_address_lookup("ns2.transip.eu.").unwrap(),
            vec![
                "2a01:7c8:f:c1f::195".parse::<IpAddr>().unwrap(),
            ],
        );
    }

    #[test]
    fn test_domain_ns() {
        let mut domain = nameservers_lookup("paulmin.nl").unwrap();
        domain.sort();
        assert_eq!(
            domain,
            vec![
                "ns0.transip.net.".to_owned(),
                "ns1.transip.nl.".to_owned(),
                "ns2.transip.eu.".to_owned(),
            ],
        );
    }
}