use serde::{Deserialize, Serialize};
use crate::{Result, api_client::ApiClient};

pub trait TransipApiGeneral {
    fn api_test(&mut self) -> Result<String>;
    fn availability_zones(&mut self) -> Result<Vec<AvailabilityZone>>;
    fn invoice_list(&mut self) -> Result<Vec<Invoice>>;
    fn products(&mut self) -> Result<Products>;
    fn product_elements(&mut self, name: &str) -> Result<Vec<ProductElement>>;
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub invoice_number: String,
    pub creation_date: String,
    pub pay_date: String,
    pub due_date: String,
    pub invoice_status: String,
    pub currency: String,
    pub total_amount: u64,
    pub total_amount_incl_vat: u64,
}

impl std::fmt::Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invoice: {}", self.invoice_number)
    }
}

#[derive(Deserialize, Serialize)]
pub struct InvoiceList {
    pub invoices: Vec<Invoice>,
}

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

impl TransipApiGeneral for ApiClient {
    fn api_test(&mut self) -> Result<String> {
        self.get::<Ping>(&self.url.api_test()).map(|p| p.ping)
    }

    fn availability_zones(&mut self) -> Result<Vec<AvailabilityZone>> {
        self.get::<AvailabilityZones>(&self.url.availability_zones()).map(|a| a.availability_zones)
    }

    fn invoice_list(&mut self) -> Result<Vec<Invoice>> {
        self.get::<InvoiceList>(&self.url.invoices()).map(|list| list.invoices)
    }

    fn product_elements(&mut self, name: &str) -> Result<Vec<ProductElement>> {
        self.get::<ProductElements>(&self.url.product_elements(name)).map(|list| list.product_elements)
    }

    fn products(&mut self) -> Result<Products> {
        self.get::<ProductList>(&self.url.products()).map(|list| list.products)
    }
}