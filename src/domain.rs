use serde::{Deserialize, Serialize};

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

impl std::fmt::Display for NameServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nameserver: {}", self.hostname)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Domain {
    name: String,
    nameservers: Vec<NameServer>,
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