use core::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::{Result, api_client::ApiClient, url::Url};

const INVOICES: &str = "invoices";

trait UrlAccount {
    fn invoice(&self, invoice_number: String) -> String;
    fn invoices(&self) -> String;
}

pub trait TransipApiAccount {
    fn invoice_list(&mut self) -> Result<Vec<Invoice>>;
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

#[derive(Deserialize, Serialize)]
pub struct InvoiceList {
    pub invoices: Vec<Invoice>,
}

impl Display for Invoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invoice: {}", self.invoice_number)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceItem {
    pub product: String,
    pub description: String,
    pub is_recurring: bool,
    pub date: String,
    pub quantity: u32,
    pub price: u32,
    pub price_incl_vat: u32,
    pub vat: u32,
    pub vat_percentage: u32,
    pub discounts: Vec<InvoiceItemDiscount>,
}

#[derive(Deserialize, Serialize)]
pub struct InvoiceItemDiscount {
    description: String,
    amount: u32,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceItemList {
    pub invoice_items: Vec<InvoiceItem>,
}

#[derive(Deserialize, Serialize)]
pub struct Pdf {
    pdf: String,
}

impl UrlAccount for Url {
    fn invoice(&self, invoice_number: String) -> String {
        format!("{}/{}", self.invoices(), invoice_number)
    }
    
    fn invoices(&self) -> String { 
        format!("{}{}", self.prefix, INVOICES) 
    }    
}

impl TransipApiAccount for ApiClient {
    fn invoice_list(&mut self) -> Result<Vec<Invoice>> {
        self.get::<InvoiceList>(&self.url.invoices()).map(|list| list.invoices)
    }
}
