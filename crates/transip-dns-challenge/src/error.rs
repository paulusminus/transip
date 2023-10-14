use tracing::subscriber::SetGlobalDefaultError;
use tracing_log::log_tracer::SetLoggerError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Api {0}")]
    Api(#[from] transip_api::Error),

    #[error("Log {0}")]
    Log(#[from] SetLoggerError),

    #[error("Dns {0}")]
    Dns(Box<dyn std::error::Error + Send + 'static>),

    #[error("Subscriber {0}")]
    Suscriber(#[from] SetGlobalDefaultError),

    #[error("{0} not found")]
    Missing(String),
}
