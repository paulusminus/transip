use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Ping {
    pub ping: String,
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
pub struct Invoices {
    pub invoices: Vec<Invoice>,
}