use std::path::Path;

use crate::{Configuration, Error, Result};

const TRANSIP_API_PRIVATE_KEY: &str = "TRANSIP_API_PRIVATE_KEY";
const TRANSIP_API_TOKEN_FILE: &str = "TRANSIP_API_TOKEN_FILE";
const TRANSIP_API_LOG_DIR: &str = "TRANSIP_API_LOG_DIR";
const TRANSIP_API_LOG_NAME: &str = "TRANSIP_API_LOG_NAME";
const TRANSIP_API_USERNAME: &str = "TRANSIP_API_USERNAME";

const ENVIRONMENT_VARIABLES: [&str; 5] = [
    TRANSIP_API_USERNAME,
    TRANSIP_API_TOKEN_FILE,
    TRANSIP_API_LOG_DIR,
    TRANSIP_API_LOG_NAME,
    TRANSIP_API_PRIVATE_KEY,
];

struct Environment {
    user_name: String,
    log_dir: String,
    log_name: String,
    private_key: String,
    token_file: String,
}

impl Configuration for Environment {
    fn user_name(&self) -> String {
        self.user_name.clone()
    }

    fn log_dir(&self) -> String {
        self.log_dir.clone()
    }

    fn log_name(&self) -> String {
        self.log_name.clone()
    }

    fn private_key(&self) -> String {
        self.private_key.clone()
    }

    fn token_file(&self) -> String {
        self.token_file.clone()
    }
}

fn var(name: &'static str) -> Result<String> {
    std::env::var(name).map_err(|_| Error::Key(name))
}

fn check_environment() -> Result<()> {
    for variable in ENVIRONMENT_VARIABLES {
        if std::env::var(variable).is_err() {
            return Err(Error::EnvironmentVariable(variable.to_owned()));
        }
    }
    if Path::new(&std::env::var(TRANSIP_API_PRIVATE_KEY).unwrap())
        .try_exists()
        .is_err()
    {
        return Err(Error::EnvironmentVariable(
            "Private key not found".to_owned(),
        ));
    }
    if !Path::new(&std::env::var(TRANSIP_API_LOG_DIR).unwrap()).exists() {
        return Err(Error::EnvironmentVariable(
            "Log directory not found".to_owned(),
        ));
    }
    Ok(())
}

pub fn configuration_from_environment() -> Result<Box<dyn Configuration>> {
    check_environment()?;
    Ok(Box::new(Environment {
        user_name: var(TRANSIP_API_USERNAME)?,
        log_dir: var(TRANSIP_API_LOG_DIR)?,
        log_name: var(TRANSIP_API_LOG_NAME)?,
        private_key: var(TRANSIP_API_PRIVATE_KEY)?,
        token_file: var(TRANSIP_API_TOKEN_FILE)?,
    }))
}
