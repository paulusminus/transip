use std::error::Error;

pub use http::{Request, Response};

#[cfg(not(all(target_family = "wasm", target_os = "wasi")))]
mod ureq_client;

#[cfg(not(all(target_family = "wasm", target_os = "wasi")))]
pub use ureq_client::Client;

#[cfg(all(target_family = "wasm", target_os = "wasi"))]
mod waki_client;

#[cfg(all(target_family = "wasm", target_os = "wasi"))]
pub use waki_client::Client;

pub trait Fetch {
    fn fetch(&self, request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Box<dyn Error>>;
}

#[cfg(test)]
mod tests {}
