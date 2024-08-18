use std::error::Error;
use std::io::Read;

use http::{Request, Response};
use ureq::Agent;

pub struct Client {
    agent: Agent,
}

impl Client {
    pub fn new() -> Self {
        Self {
            agent: Agent::new(),
        }
    }

    pub fn fetch(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
        let response = self
            .agent
            .request(request.method().as_str(), &request.uri().to_string())
            .send_bytes(request.body())?;
        let status = response.status();
        let body = response.into_reader().bytes().collect::<Result<_, _>>()?;

        Response::builder()
            .status(status)
            .body(body)
            .map_err(Into::into)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
