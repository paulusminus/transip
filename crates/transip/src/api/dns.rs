use crate::Error;
use std::fmt::Display;
use std::str::FromStr;

use crate::error::ResultExt;
use crate::{
    client::{Client, Url},
    HasName, Result,
};
use serde::{Deserialize, Serialize};

const DOMAINS: &str = "domains";
const DNS: &str = "dns";
const ACME_CHALLENGE: &str = "_acme-challenge";

trait UrlDomain {
    fn domain_dns(&self, domain_name: &str) -> String;
}

/// See <https://api.transip.nl/rest/docs.html#domains>
pub trait DnsApi {
    /// See <https://api.transip.nl/rest/docs.html#domains-dns-delete>
    fn dns_entry_delete(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()>;
    /// Delete all entries which comply to Filter F
    fn dns_entry_delete_all<F>(&mut self, domain_name: &str, f: F) -> Result<()>
    where
        F: Fn(&DnsEntry) -> bool;
    /// See <https://api.transip.nl/rest/docs.html#domains-dns-get>
    fn dns_entry_list(&mut self, domain_name: &str) -> Result<Vec<DnsEntry>>;
    /// See <https://api.transip.nl/rest/docs.html#domains-dns-post>
    fn dns_entry_insert(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()>;
}

/// Example
/// ```
/// use transip::api::dns::RecordType;
///
/// dbg!(RecordType::AAAA);
/// ```
#[derive(Debug, PartialEq)]
pub enum RecordType {
    A,
    AAAA,
    ALIAS,
    CNAME,
    MX,
    NS,
    PTR,
    SOA,
    SRV,
    TXT,
}

impl Display for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for RecordType {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let c = s.trim().to_uppercase();

        macro_rules! compare {
            ($compare:expr, $record_type:expr) => {
                if $compare == $record_type.to_string() {
                    return Ok($record_type);
                }
            };
        }

        compare!(c, RecordType::A);
        compare!(c, RecordType::AAAA);
        compare!(c, RecordType::ALIAS);
        compare!(c, RecordType::CNAME);
        compare!(c, RecordType::MX);
        compare!(c, RecordType::NS);
        compare!(c, RecordType::PTR);
        compare!(c, RecordType::SOA);
        compare!(c, RecordType::SRV);
        compare!(c, RecordType::TXT);

        Err(Error::ParseDnsEntry("invalid record type"))
    }
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DnsEntry {
    pub name: String,
    pub expire: u32,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub content: String,
}

impl DnsEntry {
    pub fn is_acme_challenge(&self) -> bool {
        self.entry_type == *"TXT" && self.name == *ACME_CHALLENGE
    }

    pub fn new_acme_challenge(expire: u32, content: &str) -> Self {
        Self {
            name: ACME_CHALLENGE.to_owned(),
            expire,
            entry_type: RecordType::TXT.to_string(),
            content: content.to_owned(),
        }
    }
}

impl std::fmt::Display for DnsEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.name, self.expire, self.entry_type, self.content
        )
    }
}

impl FromStr for DnsEntry {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut splitted = s.split(' ');
        let name = splitted
            .nth(0)
            .ok_or(Error::ParseDnsEntry("name missing"))
            .map(String::from)?;
        let expire = splitted
            .nth(1)
            .ok_or(Error::ParseDnsEntry("Second part missing"))
            .and_then(|s| s.parse::<u32>().err_into())?;
        let entry_type = splitted
            .nth(2)
            .ok_or(Error::ParseDnsEntry("name missing"))
            .and_then(|s| s.parse::<RecordType>())
            .map(|r| r.to_string())?;
        let content = splitted.skip(3).collect::<Vec<_>>().join(" ");
        if content.is_empty() {
            return Err(Error::ParseDnsEntry("content missing"));
        }
        Ok(Self {
            name,
            expire,
            entry_type,
            content,
        })
    }
}

impl HasName for DnsEntry {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsEntryList {
    pub dns_entries: Vec<DnsEntry>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DnsEntryItem {
    pub dns_entry: DnsEntry,
}

impl From<DnsEntry> for DnsEntryItem {
    fn from(dns_entry: DnsEntry) -> Self {
        Self { dns_entry }
    }
}

impl UrlDomain for Url {
    fn domain_dns(&self, domain_name: &str) -> String {
        format!("{}{}/{}/{}", self.prefix, DOMAINS, domain_name, DNS)
    }
}

impl DnsApi for Client {
    fn dns_entry_delete(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()> {
        self.delete::<DnsEntryItem>(&self.url.domain_dns(domain_name), entry.into())
    }

    fn dns_entry_delete_all<F>(&mut self, domain_name: &str, f: F) -> Result<()>
    where
        F: Fn(&DnsEntry) -> bool,
    {
        for dns_entry in self.dns_entry_list(domain_name)?.into_iter().filter(f) {
            self.dns_entry_delete(domain_name, dns_entry)?;
        }
        Ok(())
    }

    fn dns_entry_list(&mut self, domain_name: &str) -> Result<Vec<DnsEntry>> {
        self.get::<DnsEntryList>(&self.url.domain_dns(domain_name))
            .map(|list| list.dns_entries)
    }

    fn dns_entry_insert(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()> {
        self.post::<DnsEntryItem>(&self.url.domain_dns(domain_name), entry.into())
    }
}

#[cfg(test)]
mod test {
    use super::{DnsApi, DnsEntry, RecordType};
    use crate::{Client, HasNames};

    #[test]
    fn acme_challenge() {
        let dns_entry = DnsEntry::new_acme_challenge(60, "Hallo");
        assert!(dns_entry.is_acme_challenge());
    }

    #[test]
    fn record_types() {
        assert_eq!(RecordType::A.to_string().as_str(), "A");
        assert_eq!(RecordType::AAAA.to_string().as_str(), "AAAA");
        assert_eq!(RecordType::ALIAS.to_string().as_str(), "ALIAS");
        assert_eq!(RecordType::CNAME.to_string().as_str(), "CNAME");
        assert_eq!(RecordType::MX.to_string().as_str(), "MX");
        assert_eq!(RecordType::NS.to_string().as_str(), "NS");
        assert_eq!(RecordType::PTR.to_string().as_str(), "PTR");
        assert_eq!(RecordType::SOA.to_string().as_str(), "SOA");
        assert_eq!(RecordType::SRV.to_string().as_str(), "SRV");
        assert_eq!(RecordType::TXT.to_string().as_str(), "TXT");
    }

    #[test]
    fn record_types_from_str() {
        assert_eq!("A".parse::<RecordType>().unwrap(), RecordType::A);
        assert_eq!("AAAA".parse::<RecordType>().unwrap(), RecordType::AAAA);
        assert_eq!("ALIAS".parse::<RecordType>().unwrap(), RecordType::ALIAS);
        assert_eq!("CNAME".parse::<RecordType>().unwrap(), RecordType::CNAME);
        assert_eq!("MX".parse::<RecordType>().unwrap(), RecordType::MX);
        assert_eq!("NS".parse::<RecordType>().unwrap(), RecordType::NS);
        assert_eq!("PTR".parse::<RecordType>().unwrap(), RecordType::PTR);
        assert_eq!("SOA".parse::<RecordType>().unwrap(), RecordType::SOA);
        assert_eq!("SRV".parse::<RecordType>().unwrap(), RecordType::SRV);
        assert_eq!("TXT".parse::<RecordType>().unwrap(), RecordType::TXT);
    }

    #[test]
    fn domain_entry_list() {
        let entry_list = Client::demo().dns_entry_list("transipdemo.be").unwrap();
        let names = entry_list.names();

        assert_eq!(
            names,
            vec![
                "*",
                "*",
                "@",
                "@",
                "@",
                "@",
                "transip-A._domainkey",
                "transip-B._domainkey",
                "transip-C._domainkey",
                "_dmarc",
            ],
        );
    }
}
