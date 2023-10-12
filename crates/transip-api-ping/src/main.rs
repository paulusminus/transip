use std::{io::stdout, process::exit};

use tracing_subscriber::{
    filter::LevelFilter, fmt::writer::BoxMakeWriter, layer::SubscriberExt, EnvFilter,
};
use transip_api::{configuration_from_environment, ApiClient, TransipApiGeneral};

mod constant;
mod messages;

fn out() -> BoxMakeWriter {
    BoxMakeWriter::new(stdout)
}

fn rolling_or_stdout() -> BoxMakeWriter {
    if let Ok(dir) = std::env::var(constant::VAR_TRANSIP_API_LOG_DIR) {
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

fn api_test(mut client: ApiClient) {
    match client.api_test() {
        Ok(ping) => {
            if ping == *"pong" {
                messages::transip_api_test_pong_received();
            } else {
                messages::transip_api_test_other_received(&ping);
            }
        }
        Err(error) => {
            messages::transip_api_test_failed(error);
        }
    }
}

fn main() {
    if let Err(error) = tracing_log::LogTracer::init() {
        messages::failed_initializing_logger(error);
        exit(1);
    }

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(rolling_or_stdout()))
        .with(env_filter);

    if let Err(error) = tracing::subscriber::set_global_default(subscriber) {
        messages::failed_set_subscriber(error);
        exit(1);
    }

    match configuration_from_environment().and_then(ApiClient::try_from) {
        Ok(client) => api_test(client),
        Err(error) => messages::failed_initializing_api_client(error),
    }
}
