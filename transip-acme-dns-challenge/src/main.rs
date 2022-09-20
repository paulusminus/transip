use std::fs::File;
use std::sync::Mutex;

use tracing::Level;
use tracing_subscriber::prelude::*;
use tracing_subscriber::filter::LevelFilter;
use transip_api::prelude::*;
use trace::VecExt;

pub const ACME_CHALLENGE: &str = "_acme-challenge";

fn main() -> Result<()> {
    let (file, mut client): (Result<File>, ApiClient) = default_account()?.into();
    let filter_layer = LevelFilter::from_level(Level::INFO);

    match tracing_journald::layer() {
        Ok(layer) => { 
            tracing_subscriber::registry::Registry::default()
            .with(layer)
            .with(filter_layer)
            .init(); 
        },
        Err(_) => { 
            if let Ok(log_file) = file {
                let layer = tracing_subscriber::fmt::layer().with_writer(Mutex::new(log_file));
                tracing_subscriber::registry::Registry::default()
                .with(layer)
                .with(filter_layer)
                .init();
            }
            else {
                tracing_subscriber::fmt::init();
            }
        },
    }


    let validation_config = certbot::ValidationConfig::new();
    tracing::info!("Certbot environment: {:#?}", validation_config);

    if client.api_test()?.as_str() != "pong" {
        return Err(Error::ApiTest);
    }

    let is_acme_challenge = |entry: &DnsEntry| entry.name == *ACME_CHALLENGE && entry.entry_type == *"TXT";

    if let Some(auth_output) = validation_config.auth_output() {
        tracing::info!("Auth output: {}", auth_output);
        if let Some(domain) = validation_config.domain() {
            client.dns_entry_delete_all(&domain, is_acme_challenge)?;
            tracing::info!("Alle acme challenges deleted from domain {}", domain);
        }
    }
    else if let Some(challenge) = validation_config.validation() {
        if let Some(domain) = validation_config.domain() {
            client.dns_entry_delete_all(&domain, is_acme_challenge)?;
            
            let dns_entry = DnsEntry { 
                name: ACME_CHALLENGE.into(), 
                expire: 60,
                entry_type: "TXT".into(),
                content: challenge, 
            };
            client.dns_entry_insert(&domain, dns_entry)?;
                
            let name_servers = 
                client
                .nameserver_list(&domain)?
                .into_iter()
                .map(|nameserver| nameserver.hostname)
                .collect::<Vec<String>>();
            name_servers.trace();
    
            match dns_check_updated::servers_have_acme_challenge(
                name_servers.iter(),
                &domain,
                ACME_CHALLENGE,
            ) {
                Ok(_) => {
                    tracing::info!("Dns servers updated");
                    println!("OK");
                },
                Err(_) => {
                    tracing::error!("Updated Dns servers not verified");
                    println!("ERR");
                },
            };
        }                
        else {
            tracing::error!("Domain not specified in environment");
            println!("ERR");
        }
    }
    else {
        tracing::error!("Challenge not specified in environment");
        println!("ERR");
    }    
    

    Ok(())
}

mod trace {
    use core::fmt::Display;

    pub trait VecExt {
        fn trace(&self);
    }
    
    impl<T> VecExt for Vec<T> where T: Display {
        fn trace(&self) {
            self.iter().for_each(trace_object)
        }
    }
    
    fn trace_object<T>(t: T) where T: Display {
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
            self.cerbot_validation.as_ref().map(|v| v.to_owned())
        }

        pub fn domain(&self) -> Option<String> {
            self.certbot_domain.as_ref().map(|d| d.to_owned())
        }

        pub fn auth_output(&self) -> Option<String> {
            self.cerbot_auth_output.as_ref().map(|d| d.to_owned())
        }
    }
}