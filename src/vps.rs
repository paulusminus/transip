use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Vpss {
    pub vpss: Vec<Vps>,
    #[serde(rename = "_links")]
    pub links: Vec<Link>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Vps {
    pub name: String,
    pub uuid: String,
    pub description: String,
    pub product_name: String,
    pub operating_system: String,
    pub disk_size: u128,
    pub memory_size: u128,
    pub cpus: u16,
    pub status: String,
    pub ip_address: String,
    pub mac_address: String,
    pub current_snapshots: u16,
    pub max_snapshots: u16,
    pub is_locked: bool,
    pub is_blocked: bool,
    pub is_customer_locked: bool,
    pub availability_zone: String,
    pub tags: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Link {
    pub rel: String,
    pub link: String,
}