use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Ping {
    pub ping: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProductList {
    pub products: Products,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Products {
    pub vps: Vec<Product>,
    pub vps_addon: Vec<Product>,
    pub haip: Vec<Product>,
    pub private_networks: Vec<Product>,

}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: u32,
    pub recurring_price: u32,   
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Product: {}", self.name)
    }
}

#[derive(Deserialize, Serialize)]
pub struct ProductElement {
    pub name: String,
    pub description: String,
    pub amount: u64,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductElements {
    pub product_elements: Vec<ProductElement>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityZone {
    pub name: String,
    pub country: String,
    pub is_default: bool,
}

impl std::fmt::Display for AvailabilityZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Availability zone: {}", self.name)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityZones {
    pub availability_zones: Vec<AvailabilityZone>,
}