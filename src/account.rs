use serde::{Deserialize, Serialize};

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