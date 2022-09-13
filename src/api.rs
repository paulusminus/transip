use std::fmt::Display;
use std::fs::File;
use std::str::from_utf8;
use std::time::Duration;
use std::{fs::read_dir, ffi::OsStr, path::Path, io::BufReader};

use chrono::{TimeZone, Utc};
use ring::signature::{RsaKeyPair, self};
use serde::Serialize;
use serde::de::DeserializeOwned;
use ureq::{Agent, AgentBuilder};

use crate::domain::{NameServerList, NameServer, DnsEntry, DnsEntryList, DnsEntryItem};
use crate::general::{Ping, AvailabilityZone, AvailabilityZones, Invoice, InvoiceList, ProductList, Products, ProductElement, ProductElements};
use crate::url::Url;
use crate::vps::{VpsList, self};
use crate::{Result};
use crate::error::Error;
use crate::token::Token;

const TRANSIP_API_PREFIX: &str = "https://api.transip.nl/v6/"; 
const TRANSIP_CONFIG_DIR: &str = "/home/paul/.config/transip";
const TOKEN_EXPIRATION_TIME: TokenExpiration = TokenExpiration::Minutes(1);
const AGENT_TIMEOUT_SECONDS: u64 = 30;

pub enum TokenExpiration {
    Seconds(u8),
    Minutes(u8),
    Hours(u8),
}

fn fmt(count: &u8, unit: &'static str) -> String {
    format!("{} {}", count, unit) + if count <= &1 { "" } else { "s" }
}

impl Display for TokenExpiration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TokenExpiration::Seconds(seconds) => fmt(seconds, "second"),
            TokenExpiration::Minutes(minutes) => fmt(minutes, "minute"),
            TokenExpiration::Hours(hours) => fmt(hours, "hour"),
        })
    }
}

pub trait TransipApi {
    fn api_test(&mut self) -> Result<String>;
    fn availability_zones(&mut self) -> Result<Vec<AvailabilityZone>>;
    fn dns_entry_delete(&mut self, domain_name: &str, entry: DnsEntryItem) -> Result<()>;
    fn dns_entry_list(&mut self, domain_name: &str) -> Result<Vec<DnsEntry>>;
    fn dns_entry_insert(&mut self, domain_name: &str, entry: DnsEntryItem) -> Result<()>;
    fn invoice_list(&mut self) -> Result<Vec<Invoice>>;
    fn products(&mut self) -> Result<Products>;
    fn product_elements(&mut self, name: &str) -> Result<Vec<ProductElement>>;
    fn nameserver_list(&mut self, domain_name: &str) -> Result<Vec<NameServer>>;
    fn vps_list(&mut self) -> Result<Vec<vps::Vps>>;
}

pub struct AuthConfiguration {
    pub account_name: String,
    pub key: RsaKeyPair,
}

pub fn read_rsa_key_pair_from_pem<P>(path: P) -> Result<signature::RsaKeyPair> 
where P: AsRef<Path>
{
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let keys = rustls_pemfile::pkcs8_private_keys(&mut buf_reader)?;
    if keys.len() < 1 {
        Err(Error::Key("None".to_owned()))
    }
    else {
        signature::RsaKeyPair::from_pkcs8(keys[0].as_slice())
        .map_err(|_| Error::Key("Invalid".to_owned()))
    }
}

pub fn get_default_account() -> Result<AuthConfiguration> {
    read_dir(TRANSIP_CONFIG_DIR)?
    .filter_map(|de| de.ok())
    .map(|de| de.path())
    .filter(|path| path.extension() == Some(&OsStr::new("pem")))
    .nth(0)
    .and_then(|path| {
        read_rsa_key_pair_from_pem(&path).ok().map(|key| AuthConfiguration {
            account_name: path.file_stem().unwrap_or_default().to_string_lossy().to_string(),
            key,
        })
    })
    .ok_or(Error::Key("No key for account".to_owned()))
}

pub struct ApiClient {
    pub url: Url,
    pub auth_config: AuthConfiguration,
    pub token: Option<Token>,
    pub agent: Agent,
}

impl From<AuthConfiguration> for ApiClient {
    fn from(auth_config: AuthConfiguration) -> Self {
        let url = Url::new(TRANSIP_API_PREFIX.to_owned());
        let agent = AgentBuilder::new().timeout(Duration::from_secs(AGENT_TIMEOUT_SECONDS)).build();
        let token: Option<Token> = None;
        Self {
            url,
            auth_config,
            token,
            agent,
        }        
    }
}

impl ApiClient {
    fn get_token(&self) -> Result<Token> {
        let auth_request = crate::authentication::AuthRequest::new(&self.auth_config.account_name, &TOKEN_EXPIRATION_TIME.to_string());
        let json = auth_request.json();
        let signature = crate::authentication::sign(json.as_slice(), &self.auth_config.key)?;
        tracing::info!("Json signing success");
        let auth_url = self.url.auth();
        let token_response = 
            self.agent.post(&auth_url)
            .set("Signature", &signature)
            .send_bytes(json.as_slice())?
            .into_json::<crate::authentication::TokenResponse>()?;
        tracing::info!("Received token");
        let mut splitted = token_response.token.split(".");
        if splitted.clone().count() != 3 {
            Err(Error::Token)
        }
        else {
            let input = splitted.nth(1).ok_or(Error::Token)?;
            let decoded = base64::decode_config(input, base64::URL_SAFE)?;
            let s = from_utf8(decoded.as_slice())?;
            let token_meta = ureq::serde_json::from_str::<crate::token::TokenMeta>(s)?;
            let token = crate::token::Token {
                raw: token_response.token,
                expire: Utc.timestamp_millis((token_meta.exp as i64) * 1000),
            };
            Ok(token)
        }    
    }

    fn token(&mut self) -> Result<String> {
        if self.token.as_ref().map(|t| t.expired()) != Some(false) {
            tracing::info!("Token needs to be refreshed");
            let token = self.get_token()?;
            tracing::info!("Token valid till {}", token.expire);
            self.token = Some(token);
        }
        self.token.as_ref().map(|t| t.raw.clone()).ok_or(Error::Token)
    }

    fn get<T>(&mut self, url: &str) -> Result<T>
    where T: DeserializeOwned
    {
        let token = self.token()?;
        let json = self.agent.get(url)
        .set("Authorization", &format!("Bearer {}", token))
        .call()?
        .into_json::<T>()?;
        Ok(json)
    }

    fn delete<T>(&mut self, url: &str, t: T) -> Result<()>
    where T: Serialize
    {
        let token = self.token()?;
        self.agent.delete(url)
        .set("Authorization", &format!("Bearer {}", token))
        .send_json(t)?;
        Ok(())
    }
    
    fn post<T>(&mut self, url: &str, t: T) -> Result<()>
    where T: Serialize
    {
        let token = self.token()?;
        self.agent.post(url)
        .set("Authorization", &format!("Bearer {}", token))
        .send_json(t)?;
        Ok(())
    }
}
 
impl TransipApi for ApiClient {
    fn api_test(&mut self) -> Result<String> {
        self.get::<Ping>(&self.url.api_test()).map(|p| p.ping)
    }

    fn availability_zones(&mut self) -> Result<Vec<AvailabilityZone>> {
        self.get::<AvailabilityZones>(&self.url.availability_zones()).map(|a| a.availability_zones)
    }

    fn dns_entry_delete(&mut self, domain_name: &str, entry: DnsEntryItem) -> Result<()> {
        self.delete(&self.url.domain_dns(domain_name), entry)
    }

    fn dns_entry_list(&mut self, domain_name: &str) -> Result<Vec<DnsEntry>> {
        self.get::<DnsEntryList>(&self.url.domain_dns(domain_name)).map(|list| list.dns_entries)
    }

    fn dns_entry_insert(&mut self, domain_name: &str, entry: DnsEntryItem) -> Result<()> {
        self.post(&self.url.domain_dns(domain_name), entry)
    }

    fn invoice_list(&mut self) -> Result<Vec<Invoice>> {
        self.get::<InvoiceList>(&self.url.invoices()).map(|list| list.invoices)
    }

    fn nameserver_list(&mut self, domain_name: &str) -> Result<Vec<NameServer>> {
        self.get::<NameServerList>(&self.url.domain_nameservers(domain_name)).map(|list| list.nameservers)
    }

    fn product_elements(&mut self, name: &str) -> Result<Vec<ProductElement>> {
        self.get::<ProductElements>(&self.url.product_elements(name)).map(|list| list.product_elements)
    }

    fn products(&mut self) -> Result<Products> {
        self.get::<ProductList>(&self.url.products()).map(|list| list.products)
    }

    fn vps_list(&mut self) -> Result<Vec<vps::Vps>> {
        self.get::<VpsList>(&self.url.vps()).map(|list| list.vpss)
    }
}