use crate::{
    client::{Client, Url},
    HasName, Result,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

const DOMAINS: &str = "domains";
const DOMAINS_INCLUDES: &str = "?include=nameservers,contacts";
const DNS: &str = "dns";
const NAMESERVERS: &str = "nameservers";

trait UrlDomain {
    fn domain(&self, domain_name: &str) -> String;
    fn domain_dns(&self, domain_name: &str) -> String;
    fn domain_nameservers(&self, domain_name: &str) -> String;
    fn domains(&self, includes: bool) -> String;
}

/// See <https://api.transip.nl/rest/docs.html#domains>
pub trait DomainApi {
    /// See <https://api.transip.nl/rest/docs.html#domains-domains-get>
    fn domain_list(&mut self) -> Result<Vec<Domain>>;
    /// See <https://api.transip.nl/rest/docs.html#domains-dns-delete>
    #[deprecated(
        since = "0.1.2",
        note = "This name is not consistent with the api specification. Users should instead transip::api::dns::DnsApi"
    )]
    fn dns_entry_delete(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()>;
    /// Delete all entries which comply to Filter F
    #[deprecated(
        since = "0.1.2",
        note = "This name is not consistent with the rest. Users should instead transip::api::dns::DnsApi"
    )]
    fn dns_entry_delete_all<F>(&mut self, domain_name: &str, f: F) -> Result<()>
    where
        F: Fn(&DnsEntry) -> bool;
    /// See <https://api.transip.nl/rest/docs.html#domains-dns-get>
    #[deprecated(
        since = "0.1.2",
        note = "This name is not consistent with the rest. Users should instead transip::api::dns::DnsApi"
    )]
    fn dns_entry_list(&mut self, domain_name: &str) -> Result<Vec<DnsEntry>>;
    /// See <https://api.transip.nl/rest/docs.html#domains-dns-post>
    #[deprecated(
        since = "0.1.2",
        note = "This name is not consistent with the rest. Users should instead transip::api::dns::DnsApi"
    )]
    fn dns_entry_insert(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()>;
    /// See <https://api.transip.nl/rest/docs.html#domains-nameservers-get>
    fn nameserver_list(&mut self, domain_name: &str) -> Result<Vec<NameServer>>;
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WhoisContact {
    #[serde(rename = "type")]
    pub contact_type: String,
    pub first_name: String,
    pub last_name: String,
    pub company_name: String,
    pub company_kvk: String,
    pub company_type: String,
    pub street: String,
    pub number: String,
    pub postal_code: String,
    pub city: String,
    pub phone_number: String,
    pub fax_number: String,
    pub email: String,
    pub country: String,
}

#[derive(Deserialize, Serialize)]
pub struct NameServerList {
    pub nameservers: Vec<NameServer>,
}

#[derive(Deserialize, Serialize)]
pub struct NameServer {
    pub hostname: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}

impl Display for NameServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nameserver: {}", self.hostname)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
    pub name: String,
    pub nameservers: Vec<NameServer>,
    pub contacts: Vec<WhoisContact>,
    pub auth_code: Option<String>,
    pub is_transfer_locked: bool,
    pub registration_date: String,
    pub renewal_date: String,
    pub is_whitelabel: bool,
    pub cancellation_date: Option<String>,
    pub cancellation_status: Option<String>,
    pub is_dns_only: bool,
    pub tags: Vec<String>,
    pub can_edit_dns: bool,
    pub has_auto_dns: bool,
    pub has_dns_sec: bool,
    pub status: String,
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Domain: {}", self.name)
    }
}

impl HasName for Domain {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Deserialize, Serialize)]
pub struct DomainList {
    domains: Vec<Domain>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct DnsEntry {
    pub name: String,
    pub expire: u32,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub content: String,
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
    fn domain(&self, domain_name: &str) -> String {
        format!("{}/{}", self.domains(false), domain_name)
    }

    fn domain_dns(&self, domain_name: &str) -> String {
        format!("{}/{}/{}", self.domains(false), domain_name, DNS)
    }

    fn domain_nameservers(&self, domain_name: &str) -> String {
        format!("{}/{}/{}", self.domains(false), domain_name, NAMESERVERS)
    }

    fn domains(&self, includes: bool) -> String {
        format!(
            "{}{}{}",
            self.prefix,
            DOMAINS,
            if includes { DOMAINS_INCLUDES } else { "" }
        )
    }
}

impl DomainApi for Client {
    fn domain_list(&mut self) -> Result<Vec<Domain>> {
        self.get::<DomainList>(&self.url.domains(true))
            .map(|list| list.domains)
    }

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

    fn nameserver_list(&mut self, domain_name: &str) -> Result<Vec<NameServer>> {
        self.get::<NameServerList>(&self.url.domain_nameservers(domain_name))
            .map(|list| list.nameservers)
    }
}

#[cfg(test)]
mod test {
    use super::DomainApi;
    use crate::{Client, HasNames};

    #[test]
    fn domains() {
        let domains = Client::demo().domain_list().unwrap();
        let names = domains.names();
        assert_eq!(
            names,
            vec![
                "transipdemo.be",
                "transipdemo.de",
                "transipdemo.net",
                "transipdemonstratie.com",
                "transipdemonstratie.nl",
            ]
        );
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
