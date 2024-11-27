use std::{error::Error, io::Read};

pub use http::{Request, Response};

#[cfg(feature = "json")]
use serde::de::DeserializeOwned;

#[cfg(not(all(target_family = "wasm", target_env = "p2")))]
mod ureq_client;

use serde::Serialize;
#[cfg(not(all(target_family = "wasm", target_env = "p2")))]
pub use ureq_client::Client;

#[cfg(all(target_family = "wasm", target_env = "p2"))]
mod waki_client;

#[cfg(all(target_family = "wasm", target_env = "p2"))]
pub use waki_client::Client;

pub trait Fetch {
    fn fetch(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Box<dyn Error>>;
}

#[cfg(feature = "json")]
pub trait JsonApi: Fetch {
    fn get<D: DeserializeOwned>(&self, url: &str) -> Result<D, Box<dyn Error>> {
        Request::get(url)
            .header("Accept", "application/json")
            .body(Vec::<u8>::new())
            .map_err(Into::into)
            .and_then(|request| self.fetch(request))
            .and_then(|response| {
                response
                    .into_body()
                    .bytes()
                    .collect::<Result<Vec<u8>, std::io::Error>>()
                    .map_err(Into::into)
            })
            .and_then(|bytes| serde_json::from_slice(&bytes).map_err(Into::into))
    }

    fn post<S: Serialize>(&self, url: &str, s: S) -> Result<(), Box<dyn Error>> {
        serde_json::to_vec(&s)
            .map_err(Into::into)
            .and_then(|body| {
                Request::post(url)
                    .header("Content-Type", "application/json")
                    .body(body)
                    .map_err(Into::into)
            })
            .and_then(|request| self.fetch(request).map(|_| ()))
    }

    fn put<S: Serialize>(&self, url: &str, s: S) -> Result<(), Box<dyn Error>> {
        serde_json::to_vec(&s)
            .map_err(Into::into)
            .and_then(|body| {
                Request::put(url)
                    .header("Content-Type", "application/json")
                    .body(body)
                    .map_err(Into::into)
            })
            .and_then(|request| self.fetch(request).map(|_| ()))
    }

    fn delete(&self, url: &str) -> Result<(), Box<dyn Error>> {
        Request::delete(url)
            .body(Vec::<u8>::new())
            .map_err(Into::into)
            .and_then(|request| self.fetch(request).map(|_| ()))
    }
}

#[cfg(feature = "json")]
impl JsonApi for Client {}

#[cfg(test)]
mod tests {}
