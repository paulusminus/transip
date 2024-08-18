use std::{error::Error, io::Read};

pub use http::{Request, Response};

#[cfg(feature = "json")]
use serde::de::DeserializeOwned;

#[cfg(not(all(target_family = "wasm", target_os = "wasi")))]
mod ureq_client;

use serde::Serialize;
#[cfg(not(all(target_family = "wasm", target_os = "wasi")))]
pub use ureq_client::Client;

#[cfg(all(target_family = "wasm", target_os = "wasi"))]
mod waki_client;

#[cfg(all(target_family = "wasm", target_os = "wasi"))]
pub use waki_client::Client;

pub trait Fetch {
    fn fetch(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Box<dyn Error>>;
}

#[cfg(feature = "json")]
pub trait JsonApi: Fetch {
    fn get<D: DeserializeOwned>(&self, url: &str) -> Result<D, Box<dyn Error>> {
        let request = Request::get(url).header("Accept", "application/json").body(Vec::<u8>::new())?;
        let response = self.fetch(request)?;
        let bytes = response.into_body().bytes().collect::<Result<Vec<u8>, std::io::Error>>()?;
        let d: D = serde_json::from_slice(&bytes)?;
        Ok(d)
    }

    fn post<S: Serialize>(&self, url: &str, s: S) -> Result<(), Box<dyn Error>> {
        let body = serde_json::to_string(&s)?;
        let request = Request::post(url).header("Content-Type", "application/json").body(body.as_bytes().to_vec())?;
        let response = self.fetch(request)?;
        let status_code = response.status().as_u16();
        if status_code >= 400 {
            Err(format!("Status code: {}", status_code).into())
        }
        else {
            Ok(())
        }
    }

    fn put<S: Serialize>(&self, url: &str, s: S) -> Result<(), Box<dyn Error>> {
        let body = serde_json::to_string(&s)?;
        let request = Request::put(url).header("Content-Type", "application/json").body(body.as_bytes().to_vec())?;
        let response = self.fetch(request)?;
        let status_code = response.status().as_u16();
        if status_code >= 400 {
            Err(format!("Status code: {}", status_code).into())
        }
        else {
            Ok(())
        }
    }

    fn delete(&self, url: &str) -> Result<(), Box<dyn Error>> {
        let request = Request::delete(url).body(Vec::<u8>::new())?;
        let response = self.fetch(request)?;
        let status_code = response.status().as_u16();
        if status_code >= 400 {
            Err(format!("Status code: {}", status_code).into())
        }
        else {
            Ok(())
        }
    }
}

#[cfg(feature = "json")]
impl JsonApi for Client {}

#[cfg(test)]
mod tests {}
