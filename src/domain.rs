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
pub struct NameServers {
    pub nameservers: Vec<NameServer>,
}

#[derive(Deserialize, Serialize)]
pub struct NameServer {
    pub hostname: String,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Domain {
    name: String,
    nameservers: Vec<NameServer>,
}

#[derive(Deserialize, Serialize)]
pub struct DnsEntry {
    pub name: String,
    pub expire: u32,
    #[serde(rename = "type")]
    pub entry_type: String,
    pub content: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsEntries {
    pub dns_entries: Vec<DnsEntry>
}