use std::{io::stdout, process::exit, time::Instant};

use trace::VecExt;
use tracing::{error, info};
use tracing_log::LogTracer;
use tracing_subscriber::{
    filter::LevelFilter,
    fmt::{time::LocalTime, writer::BoxMakeWriter},
    prelude::*,
    EnvFilter,
};
use transip_api::{
    configuration_from_environment, ApiClient, DnsEntry, Error, TransipApiDomain, TransipApiGeneral,
};

mod constant;
mod error;

fn is_acme_challenge(entry: &DnsEntry) -> bool {
    entry.name == *constant::ACME_CHALLENGE && entry.entry_type == *"TXT"
}

fn update_dns() -> Result<(), error::Error> {
    let validation_config = certbot::ValidationConfig::new();
    info!("Certbot environment: {}", validation_config);

    let transip_domain = validation_config
        .domain()
        .ok_or(Error::EnvironmentVariable(
            constant::CERTBOT_DOMAIN.to_owned(),
        ))?;

    let mut client = configuration_from_environment().and_then(ApiClient::try_from)?;
    let _ping = client.api_test()?;

    if let Some(challenge) = validation_config.validation() {
        info!("Acme challenge {} detected", &challenge);
        let dns_entry = DnsEntry {
            name: constant::ACME_CHALLENGE.into(),
            expire: 60,
            entry_type: "TXT".into(),
            content: challenge.clone(),
        };
        client.dns_entry_insert(&transip_domain, dns_entry)?;

        let name_servers = client
            .nameserver_list(&transip_domain)?
            .into_iter()
            .map(|nameserver| nameserver.hostname)
            .collect::<Vec<String>>();
        name_servers.trace();

        dns_check_updated::servers_have_acme_challenge(
            name_servers.iter(),
            &transip_domain,
            constant::ACME_CHALLENGE,
            &challenge,
        )
        .map_err(|error| error::Error::Dns(Box::new(error)))
    } else {
        info!("Deleting all _acme_challenge records");
        client
            .dns_entry_delete_all(&transip_domain, is_acme_challenge)
            .map_err(error::Error::from)
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

fn main() {
    let start = Instant::now();
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
            let mut add_if_ok = |name: &'static str| {
                if let Ok(value) = var(name) {
                    hash_map.insert(name, value);
                }
            };
            add_if_ok(CERTBOT_DOMAIN);
            add_if_ok(CERTBOT_VALIDATION);
            add_if_ok(CERTBOT_TOKEN);
            add_if_ok(CERTBOT_REMAINING_CHALLENGES);
            add_if_ok(CERTBOT_ALL_DOMAINS);
            add_if_ok(CERTBOT_AUTH_OUTPUT);
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
