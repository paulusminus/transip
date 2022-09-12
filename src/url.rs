const API_TEST: &str = "api-test";
const AUTH: &str = "auth";
const INVOICES: &str = "invoices";
const DOMAINS: &str = "domains";
const DNS: &str = "dns";
const NAMESERVERS: &str = "nameservers";
const VPS: &str = "vps";

pub struct Url {
    prefix: String,
}

impl Url {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }

    pub fn auth(&self) -> String {
        format!("{}{}", self.prefix, AUTH)
    }

    pub fn api_test(&self) -> String {
        format!("{}{}", self.prefix, API_TEST)
    }

    pub fn invoices(&self) -> String { 
        format!("{}{}", self.prefix, INVOICES) 
    }
    
    pub fn invoice(&self, invoice_number: String) -> String {
        format!("{}/{}", self.invoices(), invoice_number)
    }

    pub fn domains(&self) -> String {
        format!("{}{}", self.prefix, DOMAINS)
    }

    pub fn domain(&self, domain_name: &str) -> String {
        format!("{}/{}", self.domains(), domain_name)
    }

    pub fn domain_dns(&self, domain_name: &str) -> String {
        format!("{}/{}/{}", self.domains(), domain_name, DNS)
    }

    pub fn domain_nameservers(&self, domain_name: &str) -> String {
        format!("{}/{}/{}", self.domains(), domain_name, NAMESERVERS)
    }

    pub fn vps(&self) -> String {
        format!("{}{}", self.prefix, VPS)
    }
}
