pub use http::{Request, Response};

#[cfg(not(all(target_family = "wasm", target_os = "wasi")))]
mod ureq_client;

#[cfg(not(all(target_family = "wasm", target_os = "wasi")))]
pub use ureq_client::Client;

#[cfg(all(target_family = "wasm", target_os = "wasi"))]
mod waki_client;

#[cfg(all(target_family = "wasm", target_os = "wasi"))]
pub use waki_client::Client;

#[cfg(test)]
mod tests {}
