use core::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::{Result, api_client::ApiClient};

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
    contact_type: String,
    first_name: String,
    last_name: String,
    company_name: String,
    company_kvk: String,
    company_type: String,
    street: String,
    number: String,
    postal_code: String,
    city: String,
    phone_number: String,
    fax_number: String,
    email: String,
    country: String,
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