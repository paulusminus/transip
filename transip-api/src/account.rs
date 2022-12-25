use core::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::{Result, api_client::{ApiClient, Url}};

const INVOICES: &str = "invoices";
const INVOICE_ITEMS: &str = "invoice-items";
const PDF: &str = "pdf";

trait UrlAccount {
    fn invoice(&self, invoice_number: &str) -> String;
    fn invoice_items(&self, invoice_number: &str) -> String;
    fn invoices(&self) -> String;
    fn invoice_pdf(&self, invoice_number: &str) -> String;
}

/// [Account](https://api.transip.nl/rest/docs.html#account)
pub trait TransipApiAccount {
    /// See <https://api.transip.nl/rest/docs.html#account-invoices-get-1>
    fn invoice(&mut self, invoice_number: &str) -> Result<Invoice>;
    /// See <https://api.transip.nl/rest/docs.html#account-invoiceitems-get>
    fn invoice_items(&mut self, invoice_number: &str) -> Result<Vec<InvoiceItem>>;
    /// See <https://api.transip.nl/rest/docs.html#account-invoices-get>
    fn invoice_list(&mut self) -> Result<Vec<Invoice>>;
    /// See <https://api.transip.nl/rest/docs.html#account-pdf-get>
    fn invoice_pdf(&mut self, invoice_number: &str) -> Result<String>;
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
pub struct InvoiceResponse {
    pub invoice: Invoice,
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
    fn invoice(&self, invoice_number: &str) -> String {
        format!("{}/{}", self.invoices(), invoice_number)
    }
    
    fn invoice_items(&self, invoice_number: &str) -> String {
        format!("{}/{}/{}", self.invoices(), invoice_number, INVOICE_ITEMS)
    }

    fn invoices(&self) -> String { 
        format!("{}{}", self.prefix, INVOICES) 
    }

    fn invoice_pdf(&self, invoice_number: &str) -> String {
        format!("{}/{}/{}", self.invoices(), invoice_number, PDF)
    }
}

impl TransipApiAccount for ApiClient {

    fn invoice(&mut self, invoice_number: &str) -> Result<Invoice> {
        self.get::<InvoiceResponse>(&self.url.invoice(invoice_number)).map(|item| item.invoice)
    }

    fn invoice_items(&mut self, invoice_number: &str) -> Result<Vec<InvoiceItem>> {
        self.get::<InvoiceItemList>(&self.url.invoice_items(invoice_number)).map(|list| list.invoice_items)
    }

    fn invoice_list(&mut self) -> Result<Vec<Invoice>> {
        self.get::<InvoiceList>(&self.url.invoices()).map(|list| list.invoices)
    }

    fn invoice_pdf(&mut self, invoice_number: &str) -> Result<String> {
        self.get::<Pdf>(&self.url.invoice_pdf(invoice_number)).map(|item| item.pdf)
    }
}
