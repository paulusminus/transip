use std::error::Error;
use tracing::{error, info};

use tracing_log::log_tracer::SetLoggerError;

pub fn failed_initializing_api_client<E: Error>(error: E) {
    error!("Api client initialisation failed: {}", error);
}

pub fn failed_initializing_logger(error: SetLoggerError) {
    eprintln!("Failed initializing logger: {}", error);
}

pub fn transip_api_test_failed<E: Error>(error: E) {
    error!("Transip api test failed: {}", error);
}

pub fn transip_api_test_pong_received() {
    info!("Received pong from transip api test");
}

pub fn transip_api_test_other_received(ping: &String) {
    error!("Wrong result from transip api test: {}", &ping);
}

pub fn failed_set_subscriber<E: Error>(error: E) {
    eprint!("Failed to set subscriber for log events: {}", error);
}
