use std::process::exit;

use tracing_subscriber::{EnvFilter, prelude::__tracing_subscriber_SubscriberExt};
use transip_api::{configuration_from_environment, ApiClient, TransipApiGeneral};

fn api_test(mut client: ApiClient) {
    match client.api_test() {
        Ok(ping) => {
            if ping == *"pong" {
                tracing::info!("Received the right result from transip api test");
            }
            else {
                tracing::error!("Wrong result from transip api test: {}", &ping);
            }
        }
        Err(error) => {
            tracing::error!("Transip api test failed: {}", error);
        }
    }
}

fn main() {
    if let Err(error) = tracing_log::LogTracer::init() {
        eprintln!("Failed initializing logger: {}", error);
        exit(1);
    }

    let level_filter = EnvFilter::from_default_env();

    let subscriber = tracing_subscriber::registry::Registry::default()
        .with(tracing_subscriber::fmt::layer())
        .with(level_filter);

    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        eprint!("Failed to set subscriber for log events: {}", error);
        exit(1);
    }

    match configuration_from_environment().and_then(ApiClient::try_from) {
        Ok(client) => api_test(client),
        Err(error) => tracing::error!("Api client initialisation failed: {}", error),
    }
}
