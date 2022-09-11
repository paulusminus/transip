use std::time::Duration;

use serde::{de::DeserializeOwned, Serialize};
use ureq::{Agent, AgentBuilder};
use crate:: Result;

const API_TEST: &str = "api-test";
const AUTH: &str = "auth";
const INVOICES: &str = "invoices";
const DOMAINS: &str = "domains";
const DNS: &str = "dns";
const NAMESERVERS: &str = "nameservers";
const VPS: &str = "vps";

pub struct Requester<'a> {
    url: Url<'a>,
    agent: Agent,
}

impl<'a> Requester<'a> {
    pub fn new(prefix: &'a str) -> Self {
        let url = Url::new(prefix);
        let agent = AgentBuilder::new().timeout(Duration::from_secs(30)).build();
        Self {
            url,
            agent,
        }
    }
}

pub struct Url<'a> {
    prefix: &'a str,
}

impl<'a> Url<'a> {
    pub fn new(prefix: &'a str) -> Self {
        Self { prefix }
    }

    pub fn auth(&self) -> String {
        format!("{}{}", self.prefix, API_TEST)
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

fn get<T>(token: &str, url: &str) -> Result<T>
where T: DeserializeOwned
{
    let json = ureq::get(url)
    .set("Authorization", &format!("Bearer {}", token))
    .call()?
    .into_json::<T>()?;
    Ok(json)
}

fn delete<T>(token: &str, url: &str, t: T) -> Result<()>
where T: Serialize
{
    ureq::delete(url)
    .set("Authorization", &format!("Bearer {}", token))
    .send_json(t)?;
    Ok(())
}

fn post<T>(token: &str, url: &str, t: T) -> Result<()>
where T: Serialize
{
    ureq::post(url)
    .set("Authorization", &format!("Bearer {}", token))
    .send_json(t)?;
    Ok(())
}
