use std::path::Path;

use crate::{Configuration, Error, Result};

const TRANSIP_API_PRIVATE_KEY: &str = "TRANSIP_API_PRIVATE_KEY";
const TRANSIP_API_USERNAME: &str = "TRANSIP_API_USERNAME";
const TRANSIP_API_TOKEN_PATH: &str = "TRANSIP_API_TOKEN_PATH";

const ENVIRONMENT_VARIABLES: [&str; 3] = [TRANSIP_API_USERNAME, TRANSIP_API_PRIVATE_KEY, TRANSIP_API_TOKEN_PATH];

struct Environment {
    user_name: String,
    private_key: String,
    token_path: String,
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
}

fn var(name: &'static str) -> Result<String> {
    std::env::var(name).map_err(|_| Error::Key(name))
}

fn check_environment() -> Result<()> {
    for variable in ENVIRONMENT_VARIABLES {
        std::env::var(variable)
            .map_err(|_| Error::EnvironmentVariable(variable.to_owned()))?;
    }
    if Path::new(&std::env::var(TRANSIP_API_PRIVATE_KEY).unwrap())
        .try_exists()
        .is_err()
    {
        return Err(Error::EnvironmentVariable(
            "Private key not found".to_owned(),
        ));
    }
    Ok(())
}

pub fn configuration_from_environment() -> Result<Box<dyn Configuration>> {
    check_environment()?;
    Ok(Box::new(Environment {
        user_name: var(TRANSIP_API_USERNAME)?,
        private_key: var(TRANSIP_API_PRIVATE_KEY)?,
        token_path: var(TRANSIP_API_TOKEN_PATH)?,
    }))
}

#[cfg(test)]
pub fn demo_configuration() -> Box<dyn Configuration> {
    Box::new(
        Environment {
            user_name: Default::default(),
            private_key: Default::default(),
            token_path: Default::default(),
        }
    )
}