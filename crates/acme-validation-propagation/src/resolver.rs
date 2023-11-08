use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use hickory_resolver::{
    config::{
        LookupIpStrategy, NameServerConfigGroup, ResolverConfig, ResolverOpts, CLOUDFLARE_IPS,
        GOOGLE_IPS,
    },
    error::ResolveErrorKind,
    lookup::{Ipv4Lookup, Ipv6Lookup},
    proto::rr::rdata::{A, AAAA},
    Resolver,
};

use crate::{recursive_resolver, Error};

pub(crate) enum ResolverType {
    Google,
    #[allow(dead_code)]
    Cloudflare,
    #[allow(dead_code)]
    Local,
}

impl ResolverType {
    fn nameservers(&self) -> &[IpAddr] {
        match self {
            ResolverType::Google => GOOGLE_IPS,
            ResolverType::Cloudflare => CLOUDFLARE_IPS,
            ResolverType::Local => &[
                IpAddr::V6(Ipv6Addr::LOCALHOST),
                IpAddr::V4(Ipv4Addr::LOCALHOST),
            ],
        }
    }

    pub fn resolver(&self, ipv6_only: bool) -> Result<Resolver, Error> {
        match self {
            ResolverType::Google => recursive_resolver(self.nameservers(), ipv6_only),
            ResolverType::Cloudflare => recursive_resolver(self.nameservers(), ipv6_only),
            ResolverType::Local => recursive_resolver(self.nameservers(), ipv6_only),
        }
    }

    pub fn recursive_resolver(&self, ipv6_only: bool) -> Result<RecursiveResolver, Error> {
        self.resolver(ipv6_only).map(RecursiveResolver::from)
    }
}

fn aaaa_to_ipv6(aaaa: AAAA) -> IpAddr {
    IpAddr::V6(*aaaa)
}

fn a_to_ipv4(a: A) -> IpAddr {
    IpAddr::V4(*a)
}

fn aaaa_mapper(f: fn(AAAA) -> IpAddr) -> impl Fn(Ipv6Lookup) -> Vec<IpAddr> {
    move |lookup| lookup.into_iter().map(f).collect()
}

fn a_mapper(f: fn(A) -> IpAddr) -> impl Fn(Ipv4Lookup) -> Vec<IpAddr> {
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

pub struct RecursiveResolver {
    inner: Resolver,
}

impl From<Resolver> for RecursiveResolver {
    fn from(resolver: Resolver) -> Self {
        Self { inner: resolver }
    }
}

impl RecursiveResolver {
    pub fn authoritive_resolvers<S>(
        &self,
        domain_name: S,
    ) -> Result<Vec<AuthoritiveResolver>, Error>
    where
        S: AsRef<str>,
    {
        self.nameservers(domain_name).and_then(|nameserver| {
            nameserver
                .into_iter()
                .map(|host_name| self.authoritive_resolver(host_name))
                .collect::<Result<Vec<AuthoritiveResolver>, Error>>()
        })
    }

    pub fn nameservers<S>(&self, domain_name: S) -> Result<Vec<String>, Error>
    where
        S: AsRef<str>,
    {
        self.inner
            .ns_lookup(domain_name.as_ref())
            .map_err(Error::from)
            .map(|lookup| lookup.into_iter().map(|ns| ns.to_string()).collect())
    }

    pub fn authoritive_resolver<S>(&self, host_name: S) -> Result<AuthoritiveResolver, Error>
    where
        S: AsRef<str>,
    {
        let ipv6_addresses = self
            .inner
            .ipv6_lookup(host_name.as_ref())
            .map_err(Error::from)
            .map(aaaa_mapper(aaaa_to_ipv6))?;

        let ipv4_addresses = self
            .inner
            .ipv4_lookup(host_name.as_ref())
            .map_err(Error::from)
            .map(a_mapper(a_to_ipv4))?;

        let ip_addresess: Vec<IpAddr> = ipv6_addresses.into_iter().chain(ipv4_addresses).collect();
        ipv6_resolver(
            NameServerConfigGroup::from_ips_clear(ip_addresess.as_slice(), 53, false),
            false,
        )
        .map(AuthoritiveResolver)
    }
}

/// Authoritive nameserver Resolver
pub struct AuthoritiveResolver(hickory_resolver::Resolver);

impl AuthoritiveResolver {
    pub fn has_single_acme<S>(&self, domain_name: S, challenge: S) -> Result<bool, Error>
    where
        S: AsRef<str>,
    {
        self.0.clear_cache();
        match self
            .0
            .txt_lookup(format!("_acme-challenge.{}", domain_name.as_ref()))
        {
            Ok(lookup) => {
                let count = lookup.iter().count();
                if count == 1 {
                    Ok(lookup
                        .iter()
                        .any(|txt| txt.to_string() == challenge.as_ref()))
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

    use crate::{error::Error, ResolverType};

    use super::RecursiveResolver;

    const DOMAIN_NAME: &str = "paulmin.nl.";

    #[test]
    fn google_nameserver() {
        let resolver = ResolverType::Google
            .resolver(true)
            .map(RecursiveResolver::from);
        assert!(resolver.is_ok());
    }

    #[test]
    fn paul_min_nl() {
        let resolver = ResolverType::Google
            .resolver(true)
            .map(RecursiveResolver::from)
            .unwrap();

        let mut names = resolver.nameservers(DOMAIN_NAME).unwrap();
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
        let resolvers = ResolverType::Google
            .resolver(true)
            .map(RecursiveResolver::from)
            .unwrap()
            .authoritive_resolvers(DOMAIN_NAME)
            .unwrap();

        let result = resolvers
            .iter()
            .map(|resolver| resolver.has_single_acme(DOMAIN_NAME, "JaJaNeeNee".into()))
            .collect::<Result<Vec<bool>, Error>>()
            .unwrap()
            .into_iter()
            .all(identity);

        assert!(result);
    }
}
