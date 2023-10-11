use std::{io::stdout, process::exit};

use tracing_subscriber::{
    filter::LevelFilter, fmt::writer::BoxMakeWriter, layer::SubscriberExt, EnvFilter,
};
use transip_api::{configuration_from_environment, ApiClient, TransipApiGeneral};

fn out() -> BoxMakeWriter {
    BoxMakeWriter::new(stdout)
}

fn rolling_or_stdout() -> BoxMakeWriter {
    if let Ok(dir) = std::env::var("TRANSIP_API_LOG_DIR") {
        if std::fs::create_dir_all(dir.as_str()).is_ok() {
            BoxMakeWriter::new(tracing_appender::rolling::daily(dir, "api.log"))
        } else {
            out()
        }
    } else {
        out()
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

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(rolling_or_stdout()))
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
