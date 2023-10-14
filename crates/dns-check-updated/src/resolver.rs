use std::net::IpAddr;

use trust_dns_resolver::{
    config::{LookupIpStrategy, NameServerConfigGroup, ResolverConfig, ResolverOpts},
    error::ResolveErrorKind,
    lookup::Ipv6Lookup,
    proto::rr::rdata::AAAA,
    Resolver,
};

use crate::Error;

// fn lookup(name: &str) -> impl Fn(Resolver) -> Result<Ipv6Lookup, Error> + '_ {
//     move |resolver| resolver.ipv6_lookup(name).map_err(Error::from)
// }

fn aaaa_to_ipv6(aaaa: AAAA) -> IpAddr {
    IpAddr::V6(*aaaa)
}

fn aaaa_mapper(f: fn(AAAA) -> IpAddr) -> impl Fn(Ipv6Lookup) -> Vec<IpAddr> {
    move |lookup| lookup.into_iter().map(f).collect()
}

fn default_ipv6_resolver_opts(recursion: bool) -> ResolverOpts {
    let mut options = ResolverOpts::default();
    options.ip_strategy = LookupIpStrategy::Ipv6Only;
    options.recursion_desired = recursion;
    options.use_hosts_file = false;
    options
}

fn ipv6_resolver(group: NameServerConfigGroup, recursion: bool) -> Result<Resolver, Error> {
    Resolver::new(
        ResolverConfig::from_parts(None, vec![], group),
        default_ipv6_resolver_opts(recursion),
    )
    .map_err(Error::from)
}

pub struct RecursiveIpv6Resolver(Resolver);

impl RecursiveIpv6Resolver {
    pub fn try_new() -> Result<Self, Error> {
        crate::google_resolver_ipv6_only().map(Self)
    }

    pub fn authoritive_ipv6_resolvers(
        &self,
        domain_name: String,
    ) -> Result<Vec<AuthoritiveIpv6Resolver>, Error> {
        self.nameservers(domain_name).and_then(|nameserver| {
            nameserver
                .into_iter()
                .map(|host_name| self.authoritive_ipv6_resolver(host_name))
                .collect::<Result<Vec<AuthoritiveIpv6Resolver>, Error>>()
        })
    }

    pub fn nameservers(&self, domain_name: String) -> Result<Vec<String>, Error> {
        self.0
            .ns_lookup(domain_name)
            .map_err(Error::from)
            .map(|lookup| lookup.into_iter().map(|ns| ns.to_string()).collect())
    }

    pub fn authoritive_ipv6_resolver(
        &self,
        host_name: String,
    ) -> Result<AuthoritiveIpv6Resolver, Error> {
        self.0
            .ipv6_lookup(host_name)
            .map_err(Error::from)
            .map(aaaa_mapper(aaaa_to_ipv6))
            .and_then(|result| {
                ipv6_resolver(
                    NameServerConfigGroup::from_ips_clear(&result, 53, false),
                    false,
                )
            })
            .map(AuthoritiveIpv6Resolver)
    }
}

/// Authoritive nameserver Resolver
pub struct AuthoritiveIpv6Resolver(trust_dns_resolver::Resolver);

impl AuthoritiveIpv6Resolver {
    pub fn has_single_acme(&self, domain_name: String, challenge: String) -> Result<bool, Error> {
        match self
            .0
            .txt_lookup(format!("_acme-challenge.{}", domain_name))
        {
            Ok(lookup) => {
                let count = lookup.iter().count();
                if count == 1 {
                    Ok(lookup.iter().any(|txt| txt.to_string() == challenge))
                } else {
                    Err(Error::MultipleAcme)
                }
            }
            Err(error) => {
                if let ResolveErrorKind::NoRecordsFound { .. } = error.kind() {
                    Ok(false)
                } else {
                    Err(Error::from(error))
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::identity;

    use crate::error::Error;

    use super::RecursiveIpv6Resolver;

    const DOMAIN_NAME: &str = "paulmin.nl.";

    #[test]
    fn google_nameserver() {
        let resolver = RecursiveIpv6Resolver::try_new();
        assert!(resolver.is_ok());
    }

    #[test]
    fn paul_min_nl() {
        let resolver = RecursiveIpv6Resolver::try_new().unwrap();

        let mut names = resolver.nameservers(DOMAIN_NAME.into()).unwrap();
        names.sort();

        assert_eq!(
            names,
            vec![
                "ns0.transip.net.".to_owned(),
                "ns1.transip.nl.".to_owned(),
                "ns2.transip.eu.".to_owned(),
            ]
        )
    }

    #[allow(dead_code)]
    fn has_acme_challenge() {
        let resolvers = {
            RecursiveIpv6Resolver::try_new()
                .unwrap()
                .authoritive_ipv6_resolvers(DOMAIN_NAME.into())
                .unwrap()
        };

        let result = resolvers
            .iter()
            .map(|resolver| resolver.has_single_acme(DOMAIN_NAME.into(), "JaJaNeeNee".into()))
            .collect::<Result<Vec<bool>, Error>>()
            .unwrap()
            .into_iter()
            .all(identity);

        assert!(result);
    }
}
