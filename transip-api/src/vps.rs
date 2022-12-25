use core::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::{Result, api_client::{ApiClient, Url}};

const VPS: &str = "vps";

trait UrlVps {
    fn vps(&self) -> String;
}

/// [VPS](https://api.transip.nl/rest/docs.html#vps)
pub trait TransipApiVps {
    /// [VPS list](https://api.transip.nl/rest/docs.html#vps-vps-get)
    fn vps_list(&mut self) -> Result<Vec<Vps>>;
}

#[derive(Deserialize, Serialize)]
pub struct VpsList {
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

impl Display for Vps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vps: {}", self.name)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Link {
    pub rel: String,
    pub link: String,
}

impl UrlVps for Url {
    fn vps(&self) -> String {
        format!("{}{}", self.prefix, VPS)
    }
}

impl TransipApiVps for ApiClient {
    fn vps_list(&mut self) -> Result<Vec<Vps>> {
        self.get::<VpsList>(&self.url.vps()).map(|list| list.vpss)
    }
}