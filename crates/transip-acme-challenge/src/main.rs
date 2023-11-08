use std::{io::stdout, process::exit, time::Instant};

use tracing::{error, info};
use tracing_log::LogTracer;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{time::LocalTime, writer::BoxMakeWriter},
    prelude::*,
    EnvFilter,
};
use transip::{
    api::{dns::DnsApi, dns::DnsEntry},
    configuration_from_environment, Client, Error,
};

mod constant;
mod error;

fn update_dns() -> Result<(), error::Error> {
    let validation_config = certbot::ValidationConfig::new();
    info!("Certbot environment: {}", validation_config);

    let transip_domain = validation_config
        .domain()
        .ok_or(Error::EnvironmentVariable(
            constant::CERTBOT_DOMAIN.to_owned(),
        ))?;

    let mut client = configuration_from_environment().and_then(Client::try_from)?;

    if args_is_cleanup() {
        info!("Deleting all _acme_challenge records");
        client
            .dns_entry_delete_all(&transip_domain, DnsEntry::is_acme_challenge)
            .map_err(error::Error::from)
    } else if let Some(challenge) = validation_config.validation() {
        info!("Acme challenge {} detected", &challenge);
        client.dns_entry_delete_all(&transip_domain, DnsEntry::is_acme_challenge)?;
        info!("All _acme-challenge records deleted");

        let dns_entry = DnsEntry {
            name: constant::ACME_CHALLENGE.into(),
            expire: 60,
            entry_type: "TXT".into(),
            content: challenge.clone(),
        };
        client.dns_entry_insert(&transip_domain, dns_entry)?;

        acme_validation_propagation::wait(format!("{transip_domain}."), challenge)
            .map_err(|error| error::Error::Dns(Box::new(error)))
    } else {
        Err(error::Error::Missing(constant::CERTBOT_VALIDATION.into()))
    }
}

fn out() -> BoxMakeWriter {
    BoxMakeWriter::new(stdout)
}

fn rolling_or_stdout() -> BoxMakeWriter {
    if let Ok(dir) = std::env::var(constant::TRANSIP_API_LOG_DIR) {
        if std::fs::create_dir_all(dir.as_str()).is_ok() {
            BoxMakeWriter::new(tracing_appender::rolling::daily(
                dir,
                constant::LOG_FILENAME_PREFIX,
            ))
        } else {
            out()
        }
    } else {
        out()
    }
}

fn run() -> Result<(), error::Error> {
    LogTracer::init_with_filter(tracing_log::log::LevelFilter::Debug)?;

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    let layer = tracing_subscriber::fmt::layer()
        .with_writer(rolling_or_stdout())
        .with_timer(LocalTime::rfc_3339());

    let subscriber = tracing_subscriber::registry().with(layer).with(env_filter);

    tracing::subscriber::set_global_default(subscriber)?;

    update_dns()
}

fn args_has_version() -> bool {
    std::env::args()
        .enumerate()
        .filter(|(i, _)| *i != 0usize)
        .any(|(_, s)| s == "-v" || s == "--version")
}

fn args_is_cleanup() -> bool {
    std::env::args()
        .enumerate()
        .filter(|(i, _)| *i != 0)
        .any(|(_, s)| s == "--cleanup")
}

fn main() {
    let start = Instant::now();
    if args_has_version() {
        println!("{}", constant::VERSION_INFO);
        return;
    }

    match run() {
        Ok(_) => {
            info!("{} seconds elapsed", start.elapsed().as_secs());
            println!("ok");
        }
        Err(error) => {
            error!("{}", error);
            info!("{} seconds elapsed", start.elapsed().as_secs());
            println!("err");
            exit(1);
        }
    }
}

mod certbot {
    use crate::constant::*;
    use std::{collections::HashMap, env::var, fmt::Display};

    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct ValidationConfig(HashMap<&'static str, String>);

    impl Display for ValidationConfig {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let values = self
                .0
                .iter()
                .map(|s| format!("{}={}", s.0, s.1))
                .collect::<Vec<_>>()
                .join(", ");
            write!(f, "{}", values)
        }
    }

    impl ValidationConfig {
        pub fn new() -> Self {
            let mut hash_map = HashMap::new();

            [
                CERTBOT_DOMAIN,
                CERTBOT_VALIDATION,
                CERTBOT_TOKEN,
                CERTBOT_REMAINING_CHALLENGES,
                CERTBOT_ALL_DOMAINS,
                CERTBOT_AUTH_OUTPUT,
            ]
            .into_iter()
            .for_each(|name| {
                if let Ok(value) = var(name) {
                    hash_map.insert(name, value);
                }
            });

            Self(hash_map)
        }

        pub fn validation(&self) -> Option<String> {
            self.0.get(CERTBOT_VALIDATION).cloned()
        }

        pub fn domain(&self) -> Option<String> {
            self.0.get(CERTBOT_DOMAIN).cloned()
        }

        #[allow(dead_code)]
        pub fn auth_output(&self) -> Option<String> {
            self.0.get(CERTBOT_AUTH_OUTPUT).cloned()
        }
    }
}
