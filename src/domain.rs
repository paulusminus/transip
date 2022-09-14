use core::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::{Result, api_client::{ApiClient, Url}};

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

pub trait TransipApiDomain {
    fn domain_list(&mut self) -> Result<Vec<Domain>>;
    fn dns_entry_delete(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()>;
    fn dns_entry_list(&mut self, domain_name: &str) -> Result<Vec<DnsEntry>>;
    fn dns_entry_insert(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()>;
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

#[derive(Deserialize, Serialize)]
pub struct DomainList {
    domains: Vec<Domain>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DnsEntry {
    pub name: String,
    pub expire: u32,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsEntryList {
    pub dns_entries: Vec<DnsEntry>
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsEntryItem {
    pub dns_entry: DnsEntry
}

impl From<DnsEntry> for DnsEntryItem {
    fn from(dns_entry: DnsEntry) -> Self {
        Self {
            dns_entry
        }
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
        format!("{}{}{}", self.prefix, DOMAINS, if includes { DOMAINS_INCLUDES } else { "" })
    }
}

impl TransipApiDomain for ApiClient {
    fn domain_list(&mut self) -> Result<Vec<Domain>> {
        self.get::<DomainList>(&self.url.domains(true)).map(|list| list.domains)
    }

    fn dns_entry_delete(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()> {
        let dns_entry_item: DnsEntryItem = entry.into();
        self.delete(&self.url.domain_dns(domain_name), dns_entry_item)
    }

    fn dns_entry_list(&mut self, domain_name: &str) -> Result<Vec<DnsEntry>> {
        self.get::<DnsEntryList>(&self.url.domain_dns(domain_name)).map(|list| list.dns_entries)
    }

    fn dns_entry_insert(&mut self, domain_name: &str, entry: DnsEntry) -> Result<()> {
        let dns_entry_item: DnsEntryItem = entry.into();
        self.post(&self.url.domain_dns(domain_name), dns_entry_item)
    }

    fn nameserver_list(&mut self, domain_name: &str) -> Result<Vec<NameServer>> {
        self.get::<NameServerList>(&self.url.domain_nameservers(domain_name)).map(|list| list.nameservers)
    }
}