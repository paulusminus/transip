const API_TEST: &str = "api-test";
const AUTH: &str = "auth";
const AVAILABILITY_ZONES: &str = "availability-zones";
const INVOICES: &str = "invoices";
const DOMAINS: &str = "domains";
const DNS: &str = "dns";
const NAMESERVERS: &str = "nameservers";
const PRODUCT_ELEMENTS: &str = "elements";
const PRODUCTS: &str = "products";

const VPS: &str = "vps";

pub struct Url {
    prefix: String,
}

impl Url {
    pub fn new(prefix: String) -> Self {
        Self { prefix }
    }

    pub fn api_test(&self) -> String {
        format!("{}{}", self.prefix, API_TEST)
    }

    pub fn auth(&self) -> String {
        format!("{}{}", self.prefix, AUTH)
    }

    pub fn availability_zones(&self) -> String {
        format!("{}{}", self.prefix, AVAILABILITY_ZONES)
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

    pub fn domains(&self) -> String {
        format!("{}{}", self.prefix, DOMAINS)
    }

    pub fn invoice(&self, invoice_number: String) -> String {
        format!("{}/{}", self.invoices(), invoice_number)
    }

    pub fn invoices(&self) -> String { 
        format!("{}{}", self.prefix, INVOICES) 
    }

    pub fn product_elements(&self, name: &str) -> String {
        format!("{}/{}", self.products(), PRODUCT_ELEMENTS)
    }

    pub fn products(&self) -> String {
        format!("{}{}", self.prefix, PRODUCTS)
    }

    pub fn vps(&self) -> String {
        format!("{}{}", self.prefix, VPS)
    }
}
