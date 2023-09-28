use std::sync::Mutex;

use trace::VecExt;
use tracing::Level;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use transip_api::*;

pub const ACME_CHALLENGE: &str = "_acme-challenge";

fn is_acme_challenge(entry: &DnsEntry) -> bool {
    entry.name == *ACME_CHALLENGE && entry.entry_type == *"TXT"
}

fn main() -> Result<()> {
    let (mut client, log_file): (ApiClient, WriteWrapper) = default_account()?.into();
    let filter_layer = LevelFilter::from_level(Level::INFO);

    match tracing_journald::layer() {
        Ok(layer) => {
            tracing_subscriber::registry::Registry::default()
                .with(layer)
                .with(filter_layer)
                .init();
        }
        Err(_) => {
            tracing_subscriber::fmt()
                .with_writer(Mutex::new(log_file))
                .with_max_level(Level::INFO)
                .init();
        }
    }

    if let Ok(transip_domain) = std::env::var("TRANSIP_DOMAIN_NAME") {
        let validation_config = certbot::ValidationConfig::new();
        tracing::info!("Certbot environment: {:#?}", validation_config);

        if client.api_test()?.as_str() != "pong" {
            return Err(Error::ApiTest);
        }

        if let Some(auth_output) = validation_config.auth_output() {
            tracing::info!("Auth output: {}", auth_output);
            if let Some(domain) = validation_config.domain() {
                client.dns_entry_delete_all(&transip_domain, is_acme_challenge)?;
                tracing::info!("Alle acme challenges deleted from domain {}", domain);
            }
        } else if let Some(challenge) = validation_config.validation() {
            if let Some(domain) = validation_config.domain() {
                client.dns_entry_delete_all(&transip_domain, is_acme_challenge)?;

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

                match dns_check_updated::servers_have_acme_challenge(
                    name_servers.iter(),
                    &transip_domain,
                    ACME_CHALLENGE,
                ) {
                    Ok(_) => {
                        tracing::info!("Dns servers updated");
                        println!("OK");
                    }
                    Err(_) => {
                        tracing::error!("Updated Dns servers not verified");
                        println!("ERR");
                    }
                };
            } else {
                tracing::error!("Domain not specified in environment");
                println!("ERR");
            }
        } else {
            tracing::error!("Challenge not specified in environment");
            println!("ERR");
        }
    } else {
        eprintln!("Environment variable TRANSIP_DOMAIN_NAME not set");
    }

    Ok(())
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
