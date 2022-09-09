use std::time::Duration;
use ureq::{Agent, AgentBuilder};

const TIMEOUT: Duration = Duration::from_secs(10);

pub struct Client {
    agent: Agent,
}

impl Default for Client {
    fn default() -> Self {
        let agent = 
        AgentBuilder::new()
        .timeout_read(TIMEOUT)
        .timeout_write(TIMEOUT)
        .build();
        Self {
            agent
        }
    }
}