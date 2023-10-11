use std::process::exit;

use trace::VecExt;
use tracing::Level;
use tracing_log::LogTracer;
use tracing_subscriber::prelude::*;
use transip_api::{
    configuration_from_environment, ApiClient, DnsEntry, Error, Result, TransipApiDomain,
    TransipApiGeneral,
};

pub const ACME_CHALLENGE: &str = "_acme-challenge";
pub const TRANSIP_DOMAIN_NAME: &str = "TRANSIP_DOMAIN_NAME";

fn is_acme_challenge(entry: &DnsEntry) -> bool {
    entry.name == *ACME_CHALLENGE && entry.entry_type == *"TXT"
}

fn update_dns() -> Result<()> {
    let transip_domain = std::env::var(TRANSIP_DOMAIN_NAME)?;
    let validation_config = certbot::ValidationConfig::new();
    tracing::info!("Certbot environment: {:#?}", validation_config);

    let mut client = configuration_from_environment().and_then(ApiClient::try_from)?;
    let ping = client.api_test()?;
    if ping.as_str() != "pong" {
        return Err(Error::ApiTest);
    }

    let auth_output = validation_config.auth_output().ok_or(Error::AcmeChallege)?;

    tracing::info!("Auth output: {}", auth_output);
    let domain = validation_config.domain().ok_or(Error::AcmeChallege)?;
    client.dns_entry_delete_all(&transip_domain, is_acme_challenge)?;
    tracing::info!("Alle acme challenges deleted from domain {}", domain);
    if let Some(challenge) = validation_config.validation() {
        let dns_entry = DnsEntry {
            name: ACME_CHALLENGE.into(),
            expire: 60,
            entry_type: "TXT".into(),
            content: challenge,
        };
        client.dns_entry_insert(&transip_domain, dns_entry)?;

        let name_servers = client
            .nameserver_list(&domain)?
            .into_iter()
            .map(|nameserver| nameserver.hostname)
            .collect::<Vec<String>>();
        name_servers.trace();

        dns_check_updated::servers_have_acme_challenge(
            name_servers.iter(),
            &transip_domain,
            ACME_CHALLENGE,
        )
        .map_err(|_| Error::AcmeChallege)?;
    }
    Ok(())
}

fn main() {
    if let Err(error) = LogTracer::init_with_filter(tracing_log::log::LevelFilter::Debug) {
        eprint!("Error: {}", error);
        exit(1);
    }

    let filter_layer = tracing::level_filters::LevelFilter::from_level(Level::DEBUG);

    match tracing_journald::layer() {
        Ok(layer) => {
            let subscriber = tracing_subscriber::registry::Registry::default()
                .with(layer)
                .with(filter_layer);
            tracing::subscriber::set_global_default(subscriber).unwrap();
        }
        Err(_) => {
            let subscriber = tracing_subscriber::registry()
                .with(tracing_subscriber::fmt::layer())
                .with(filter_layer);
                tracing::subscriber::set_global_default(subscriber).unwrap();
        }
    }

    match update_dns() {
        Ok(_) => {
            println!("ok");
        }
        Err(error) => {
            tracing::error!("{}", error);
            println!("err");
            exit(1);
        }
    }
}

mod trace {
    use core::fmt::Display;

    pub trait VecExt {
        fn trace(&self);
    }

    impl<T> VecExt for Vec<T>
    where
        T: Display,
    {
        fn trace(&self) {
            self.iter().for_each(trace_object)
        }
    }

    fn trace_object<T>(t: T)
    where
        T: Display,
    {
        tracing::info!("{}", t)
    }
}

mod certbot {
    use std::env::var;

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct ValidationConfig {
        certbot_domain: Option<String>,
        cerbot_validation: Option<String>,
        cerbot_token: Option<String>,
        certbot_remaining_challenges: Option<String>,
        cerbot_all_domains: Option<String>,
        cerbot_auth_output: Option<String>,
    }

    impl ValidationConfig {
        pub fn new() -> Self {
            Self {
                certbot_domain: var("CERTBOT_DOMAIN").ok(),
                cerbot_validation: var("CERTBOT_VALIDATION").ok(),
                cerbot_token: var("CERTBOT_TOKEN").ok(),
                certbot_remaining_challenges: var("CERTBOT_REMAINING_CHALLENGENS").ok(),
                cerbot_all_domains: var("CERTBOT_ALL_DOMAINS").ok(),
                cerbot_auth_output: var("CERTBOT_AUTH_OUTPUT").ok(),
            }
        }

        pub fn validation(&self) -> Option<String> {
            self.cerbot_validation.clone()
        }

        pub fn domain(&self) -> Option<String> {
            self.certbot_domain.clone()
        }

        pub fn auth_output(&self) -> Option<String> {
            self.cerbot_auth_output.clone()
        }
    }
}
