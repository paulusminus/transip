use std::path::Path;

use crate::{Configuration, Error, Result};

const TRANSIP_API_PRIVATE_KEY: &str = "TRANSIP_API_PRIVATE_KEY";
const TRANSIP_API_USERNAME: &str = "TRANSIP_API_USERNAME";

const ENVIRONMENT_VARIABLES: [&str; 2] = [TRANSIP_API_USERNAME, TRANSIP_API_PRIVATE_KEY];

struct Environment {
    user_name: String,
    private_key: String,
}

impl Configuration for Environment {
    fn user_name(&self) -> String {
        self.user_name.clone()
    }

    fn private_key(&self) -> String {
        self.private_key.clone()
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
    Ok(())
}

pub fn configuration_from_environment() -> Result<Box<dyn Configuration>> {
    check_environment()?;
    Ok(Box::new(Environment {
        user_name: var(TRANSIP_API_USERNAME)?,
        private_key: var(TRANSIP_API_PRIVATE_KEY)?,
    }))
}
