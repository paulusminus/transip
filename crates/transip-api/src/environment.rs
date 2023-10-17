use std::path::Path;

use crate::{Configuration, Error, Result};

const TRANSIP_API_PRIVATE_KEY: &str = "TRANSIP_API_PRIVATE_KEY";
const TRANSIP_API_USERNAME: &str = "TRANSIP_API_USERNAME";
const TRANSIP_API_TOKEN_PATH: &str = "TRANSIP_API_TOKEN_PATH";
const TRANSIP_API_WHITELISTED_ONLY: &str = "TRANSIP_API_WHITELISTED_ONLY";
const TRANSIP_API_READONLY: &str = "TRANSIP_API_READONLY";

const ENVIRONMENT_VARIABLES: [&str; 5] = [
    TRANSIP_API_USERNAME,
    TRANSIP_API_PRIVATE_KEY,
    TRANSIP_API_TOKEN_PATH,
    TRANSIP_API_WHITELISTED_ONLY,
    TRANSIP_API_READONLY,
];

struct Environment {
    user_name: String,
    private_key: String,
    token_path: String,
    whitelisted_only: bool,
    read_only: bool,
}

impl Configuration for Environment {
    fn user_name(&self) -> &str {
        self.user_name.as_str()
    }

    fn private_key_pem_file(&self) -> &str {
        self.private_key.as_str()
    }

    fn token_path(&self) -> &str {
        self.token_path.as_str()
    }

    fn whitelisted_only(&self) -> bool {
        self.whitelisted_only
    }

    fn read_only(&self) -> bool {
        self.read_only
    }
}

fn var(name: &'static str) -> Result<String> {
    std::env::var(name).map_err(|_| Error::Key(name))
}

fn check_environment() -> Result<()> {
    for variable in ENVIRONMENT_VARIABLES {
        std::env::var(variable).map_err(|_| Error::EnvironmentVariable(variable.to_owned()))?;
    }
    if Path::new(&std::env::var(TRANSIP_API_PRIVATE_KEY).unwrap())
        .try_exists()
        .is_err()
    {
        return Err(Error::EnvironmentVariable(
            "Private key not found".to_owned(),
        ));
    }
    if var(TRANSIP_API_WHITELISTED_ONLY)
        .unwrap()
        .parse::<bool>()
        .is_err()
    {
        return Err(Error::EnvironmentVariable(format!(
            "{} should contain true of false",
            TRANSIP_API_WHITELISTED_ONLY
        )));
    }
    if var(TRANSIP_API_READONLY).unwrap().parse::<bool>().is_err() {
        return Err(Error::EnvironmentVariable(format!(
            "{} should contain true of false",
            TRANSIP_API_READONLY
        )));
    }
    Ok(())
}

pub fn configuration_from_environment() -> Result<Box<dyn Configuration>> {
    check_environment()?;
    Ok(Box::new(Environment {
        user_name: var(TRANSIP_API_USERNAME)?,
        private_key: var(TRANSIP_API_PRIVATE_KEY)?,
        token_path: var(TRANSIP_API_TOKEN_PATH)?,
        whitelisted_only: var(TRANSIP_API_WHITELISTED_ONLY).and_then(parse_boolean)?,
        read_only: var(TRANSIP_API_READONLY).and_then(parse_boolean)?,
    }))
}

fn parse_boolean(s: String) -> Result<bool> {
    s.parse::<bool>().map_err(Into::into)
}

#[cfg(test)]
pub fn demo_configuration() -> Box<dyn Configuration> {
    Box::new(Environment {
        user_name: Default::default(),
        private_key: Default::default(),
        token_path: Default::default(),
        read_only: false,
        whitelisted_only: false,
    })
}
