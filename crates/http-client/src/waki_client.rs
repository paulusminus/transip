use std::error::Error;
use waki::Client as WakiClient;

use http::{Request, Response};

use crate::Fetch;

pub struct Client {
    agent: WakiClient,
}

impl Client {
    pub fn new() -> Self {
        Self {
            agent: WakiClient::new(),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

impl Fetch for Client {
    fn fetch(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
        let method = waki_method(request.method())?;
        let body = request.body().clone();
        let response = self
            .agent
            .request(method, request.uri().to_string().as_str())
            .body(body)
            .send()?;
        let status = response.status_code();
        let response_body = response.body()?;

        if status >= 400 {
            Err(format!("status: {status}").as_str().into())
        } else {
            Response::builder()
                .status(status)
                .body(response_body)
                .map_err(Into::into)
        }
    }
}

fn waki_method(method: &http::Method) -> Result<waki::Method, Box<dyn Error>> {
    match method {
        &http::Method::CONNECT => Ok(waki::Method::Connect),
        &http::Method::DELETE => Ok(waki::Method::Delete),
        &http::Method::GET => Ok(waki::Method::Get),
        &http::Method::HEAD => Ok(waki::Method::Head),
        &http::Method::OPTIONS => Ok(waki::Method::Options),
        &http::Method::PATCH => Ok(waki::Method::Patch),
        &http::Method::POST => Ok(waki::Method::Post),
        &http::Method::PUT => Ok(waki::Method::Put),
        &http::Method::TRACE => Ok(waki::Method::Trace),
        _ => Err("Invalid method".into()),
    }
}
