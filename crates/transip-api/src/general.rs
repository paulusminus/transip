use crate::{
    api_client::{ApiClient, Url},
    HasName, Result,
};
use core::fmt::Display;
use serde::{Deserialize, Serialize};

const API_TEST: &str = "api-test";
const AVAILABILITY_ZONES: &str = "availability-zones";
const PRODUCT_ELEMENTS: &str = "elements";
const PRODUCTS: &str = "products";

trait UrlGeneral {
    fn api_test(&self) -> String;
    fn availability_zones(&self) -> String;
    fn products(&self) -> String;
    fn product_elements(&self, name: &str) -> String;
}

/// [General](https://api.transip.nl/rest/docs.html#general)
pub trait TransipApiGeneral {
    /// See <https://api.transip.nl/rest/docs.html#general-apitest-get>
    ///
    /// The result of this method should always be `pong` if successfull.
    ///
    /// # Example
    ///
    /// ```rust
    /// let ping_result: String = client.api_test()?;
    /// ```
    ///
    fn api_test(&mut self) -> Result<String>;

    /// See <https://api.transip.nl/rest/docs.html#general-availabilityzone-get>
    fn availability_zones(&mut self) -> Result<Vec<AvailabilityZone>>;

    /// See <https://api.transip.nl/rest/docs.html#general-products-get>
    fn products(&mut self) -> Result<Products>;

    /// See <https://api.transip.nl/rest/docs.html#general-elements-get>
    fn product_elements(&mut self, name: &str) -> Result<Vec<ProductElement>>;
}

#[derive(Deserialize, Serialize)]
struct Ping {
    pub ping: String,
}

#[derive(Deserialize, Serialize)]
struct ProductList {
    pub products: Products,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Products {
    pub vps: Vec<Product>,
    pub vps_addon: Vec<Product>,
    pub haip: Vec<Product>,
    pub private_networks: Vec<Product>,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub name: String,
    pub description: String,
    pub price: u32,
    pub recurring_price: u32,
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Product: {}", self.name)
    }
}

impl HasName for Product {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Deserialize, Serialize, PartialEq)]
pub struct ProductElement {
    pub name: String,
    pub description: String,
    pub amount: u64,
}

impl HasName for ProductElement {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Display for ProductElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Element: {}, {} = {}",
            self.name, self.description, self.amount
        )
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductElements {
    pub product_elements: Vec<ProductElement>,
}

/// What is Availability
#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvailabilityZone {
    pub name: String,
    pub country: String,
    pub is_default: bool,
}

impl HasName for AvailabilityZone {
    fn name(&self) -> &str {
        self.name.as_str()
    }
}

impl Display for AvailabilityZone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Availability zone: {}", self.name)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct AvailabilityZones {
    pub availability_zones: Vec<AvailabilityZone>,
}

impl UrlGeneral for Url {
    fn api_test(&self) -> String {
        format!("{}{}", self.prefix, API_TEST)
    }

    fn availability_zones(&self) -> String {
        format!("{}{}", self.prefix, AVAILABILITY_ZONES)
    }

    fn product_elements(&self, name: &str) -> String {
        format!("{}/{}/{}", self.products(), name, PRODUCT_ELEMENTS)
    }

    fn products(&self) -> String {
        format!("{}{}", self.prefix, PRODUCTS)
    }
}

impl TransipApiGeneral for ApiClient {
    fn api_test(&mut self) -> Result<String> {
        self.get::<Ping>(&self.url.api_test()).map(|p| p.ping)
    }

    fn availability_zones(&mut self) -> Result<Vec<AvailabilityZone>> {
        self.get::<AvailabilityZones>(&self.url.availability_zones())
            .map(|a| a.availability_zones)
    }

    fn product_elements(&mut self, name: &str) -> Result<Vec<ProductElement>> {
        self.get::<ProductElements>(&self.url.product_elements(name))
            .map(|list| list.product_elements)
    }

    fn products(&mut self) -> Result<Products> {
        self.get::<ProductList>(&self.url.products())
            .map(|list| list.products)
    }
}

#[cfg(test)]
mod tests {
    use super::TransipApiGeneral;
    use crate::HasNames;
    use crate::{api_client::ApiClient, general::Product};

    #[test]
    fn api_test() {
        let ping = ApiClient::demo().api_test().unwrap();
        assert_eq!(ping, "pong".to_owned());
    }

    #[test]
    fn availability_zones() {
        let zones = ApiClient::demo().availability_zones().unwrap();
        let names = zones.names();
        assert_eq!(names, vec!["ams0", "rtm0",],);
    }

    #[test]
    fn vps_products() {
        let products = ApiClient::demo().products().unwrap().vps;
        let names = products.names();
        assert_eq!(
            names,
            vec![
                "vps-bladevps-xs",
                "vps-bladevps-x2",
                "vps-bladevps-x4",
                "vps-bladevps-x8",
                "vps-bladevps-pro-x16",
                "vps-bladevps-pro-x24",
                "vps-bladevps-pro-x64",
                "vps-performance-c2",
                "vps-performance-c4",
                "vps-performance-c8",
                "vps-performance-c16",
                "vps-performance-c32",
                "vps-sandbox-d1",
                "vps-sandbox-d2",
                "vps-sandbox-d3",
            ]
        );
    }

    #[test]
    fn haip_products() {
        let products: Vec<Product> = ApiClient::demo().products().unwrap().haip;
        let names = products.names();

        assert_eq!(names, vec!["haip-basic-contract", "haip-pro-contract",]);
    }

    #[test]
    fn haip_basic_product_elements() {
        let elements = ApiClient::demo()
            .product_elements("haip-basic-contract")
            .unwrap();
        let names = elements.names();

        assert_eq!(names, vec!["haip-load-balancing",]);
    }
}
