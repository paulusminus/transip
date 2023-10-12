use std::{io::stdout, process::exit, time::Instant};

use tracing::{error, info};
use tracing_subscriber::{
    filter::LevelFilter, fmt::time::LocalTime, fmt::writer::BoxMakeWriter, layer::SubscriberExt,
    EnvFilter,
};
use transip_api::{configuration_from_environment, ApiClient, TransipApiGeneral};

use crate::error::Error;

mod constant;
mod error;

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

fn api_test(mut client: ApiClient) -> Result<(), Error> {
    let ping = client.api_test()?;
    if ping != *"pong" {
        Err(Error::Ping(ping))
    } else {
        Ok(())
    }
}

fn run() -> Result<(), Error> {
    tracing_log::LogTracer::init()?;

    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::DEBUG.into())
        .from_env_lossy();

    let layer = tracing_subscriber::fmt::layer()
        .with_writer(rolling_or_stdout())
        .with_timer(LocalTime::rfc_3339());

    let subscriber = tracing_subscriber::registry().with(layer).with(env_filter);

    tracing::subscriber::set_global_default(subscriber)?;

    let client = configuration_from_environment().and_then(ApiClient::try_from)?;
    api_test(client)
}

fn main() {
    let start = Instant::now();
    match run() {
        Ok(_) => {
            info!("{} milliseconds elapsed", start.elapsed().as_millis());
        }
        Err(error) => {
            error!("{}", error);
            info!("{} milliseconds elapsed", start.elapsed().as_millis());
            exit(1);
        }
    }
}
