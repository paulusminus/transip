use std::process::exit;

use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Layer, Registry};
use transip_api::{configuration_from_environment, ApiClient, TransipApiGeneral};

fn choose_layer() -> Box<dyn Layer<Registry> + Send + Sync> {
    if let Ok(journald_layer) = tracing_journald::layer() {
        journald_layer.boxed()
    } else {
        tracing_subscriber::fmt::layer().boxed()
    }
}

fn api_test(mut client: ApiClient) {
    match client.api_test() {
        Ok(ping) => {
            if ping == *"pong" {
                tracing::info!("Received the right result from transip api test");
            } else {
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

    let env_filter = EnvFilter::from_default_env();

    let subscriber = tracing_subscriber::registry()
        .with(choose_layer())
        .with(env_filter);

    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        eprint!("Failed to set subscriber for log events: {}", error);
        exit(1);
    }

    match configuration_from_environment().and_then(ApiClient::try_from) {
        Ok(client) => api_test(client),
        Err(error) => tracing::error!("Api client initialisation failed: {}", error),
    }
}
