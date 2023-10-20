use crate::{
    client::{Client, Url},
    HasName, Result,
};
use core::fmt::Display;
use serde::{Deserialize, Serialize};

const VPS: &str = "vps";

trait UrlVps {
    fn vps_list(&self) -> String;
    fn vps(&self, name: &str) -> String;
}

/// [VPS](https://api.transip.nl/rest/docs.html#vps)
pub trait TransipApiVps {
    /// [VPS list](https://api.transip.nl/rest/docs.html#vps-vps-get)
    fn vps_list(&mut self) -> Result<Vec<Vps>>;

    fn vps(&mut self, name: &str) -> Result<Vps>;

    fn vps_stop(&mut self, name: &str) -> Result<()>;

    fn vps_start(&mut self, name: &str) -> Result<()>;

    fn vps_reset(&mut self, name: &str) -> Result<()>;

    fn vps_set_is_locked(&mut self, name: &str, locked: bool) -> Result<()>;

    fn vps_set_description(&mut self, name: &str, description: &str) -> Result<()>;
}

#[derive(Serialize, Debug)]
struct Action {
    action: String,
}

impl Action {
    fn stop() -> Self {
        Self {
            action: "stop".to_owned(),
        }
    }
    fn start() -> Self {
        Self {
            action: "start".to_owned(),
        }
    }
    fn reset() -> Self {
        Self {
            action: "reset".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct VpsList {
    pub vpss: Vec<Vps>,
    #[serde(rename = "_links")]
    pub links: Vec<Link>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct VpsItem {
    vps: Vps,
}

impl From<Vps> for VpsItem {
    fn from(vps: Vps) -> Self {
        Self { vps }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
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

impl HasName for Vps {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Deserialize, Serialize)]
pub struct Link {
    pub rel: String,
    pub link: String,
}

impl UrlVps for Url {
    fn vps_list(&self) -> String {
        format!("{}{}", self.prefix, VPS)
    }

    fn vps(&self, name: &str) -> String {
        format!("{}{}/{}", self.prefix, VPS, name)
    }
}

impl TransipApiVps for Client {
    fn vps_list(&mut self) -> Result<Vec<Vps>> {
        self.get::<VpsList>(&self.url.vps_list())
            .map(|list| list.vpss)
    }

    fn vps(&mut self, name: &str) -> Result<Vps> {
        self.get::<VpsItem>(&self.url.vps(name))
            .map(|item| item.vps)
    }

    fn vps_stop(&mut self, name: &str) -> Result<()> {
        self.patch(&self.url.vps(name), &Action::stop())
    }

    fn vps_start(&mut self, name: &str) -> Result<()> {
        self.patch(&self.url.vps(name), &Action::start())
    }

    fn vps_reset(&mut self, name: &str) -> Result<()> {
        self.patch(&self.url.vps(name), &Action::reset())
    }

    fn vps_set_is_locked(&mut self, name: &str, locked: bool) -> Result<()> {
        let mut vps_item = self.vps(name).map(VpsItem::from)?;
        vps_item.vps.is_customer_locked = locked;
        self.put(&self.url.vps(name), &vps_item)
    }

    fn vps_set_description(&mut self, name: &str, description: &str) -> Result<()> {
        let mut vps_item = self.vps(name).map(VpsItem::from)?;
        vps_item.vps.description = description.to_owned();
        self.put(&self.url.vps(name), &vps_item)
    }
}

#[cfg(test)]
mod test {
    use super::{TransipApiVps, Vps};
    use crate::{Client, HasNames};

    #[test]
    fn vps_list() {
        let mut client = Client::demo();
        let list = client.vps_list().unwrap();
        let vps_names = list.names();
        assert_eq!(
            vps_names,
            vec![
                "transipdemo-vps",
                "transipdemo-vps2",
                "transipdemo-vps3",
                "transipdemo-vps4",
                "transipdemo-vps5",
                "transipdemo-vps6"
            ]
        );
    }

    #[test]
    fn vps() {
        let mut client = Client::demo();
        let vps = client.vps("transipdemo-vps6").unwrap();

        assert_eq!(
            vps,
            Vps {
                name: "transipdemo-vps6".to_owned(),
                uuid: "65297796-094f-8a88-caa8-00000a7ae63d".to_owned(),
                description: "".to_owned(),
                product_name: "vps-bladevps-pro-x32".to_owned(),
                operating_system: "Plesk Onyx Web Pro Edition 17.8.11 + CentOS 7".to_owned(),
                disk_size: 1048576000,
                memory_size: 33554432,
                cpus: 6,
                status: "running".to_owned(),
                ip_address: "149.210.192.188".to_owned(),
                mac_address: "52:54:00:7a:96:03".to_owned(),
                current_snapshots: 0,
                max_snapshots: 1,
                is_locked: false,
                is_blocked: false,
                is_customer_locked: false,
                availability_zone: "ams0".to_owned(),
                tags: [].into_iter().collect::<Vec<String>>(),
            },
        );
    }
}
