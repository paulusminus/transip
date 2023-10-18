use tracing::subscriber::SetGlobalDefaultError;
use tracing_log::log_tracer::SetLoggerError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Api {0}")]
    Api(#[from] transip::Error),

    #[error("Subscriber {0}")]
    Subscriber(#[from] SetGlobalDefaultError),

    #[error("Log {0}")]
    Log(#[from] SetLoggerError),

    #[error("Ping result: {0}")]
    Ping(String),
}
