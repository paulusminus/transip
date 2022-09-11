use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Ping {
    pub ping: String,
}

#[derive(Deserialize, Serialize)]
pub struct ProductList {
    products: Products,
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