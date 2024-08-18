use std::error::Error;

use http::{Request, Response};

#[cfg(all(target_family = "wasm", target_env = "p1"))]
pub fn fetch(request: Request<Vec<u8>>) -> Result<Response<Request<Vec<u8>>>, Box<dyn Error>> {
    Response::builder()
        .status(200)
        .body(request)
        .map_err(Into::into)
}

#[cfg(not(all(target_family = "wasm", target_env = "p1")))]
pub fn fetch(request: Request<Vec<u8>>) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    use std::io::Read;

    let response = ureq::Agent::new()
        .request(request.method().as_str(), &request.uri().to_string())
        .send_bytes(request.body())?;
    let status = response.status();
    let body = response.into_reader().bytes().collect::<Result<_, _>>()?;

    Response::builder()
        .status(status)
        .body(body)
        .map_err(Into::into)
}

#[cfg(test)]
mod tests {}
