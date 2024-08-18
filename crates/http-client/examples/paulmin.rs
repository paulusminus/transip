use std::str::from_utf8;

use http_client::{Client, Fetch};

fn empty_body() -> Vec<u8> {
    vec![]
}

fn main() {
    let client = Client::default();
    let request = http::Request::get("https://www.paulmin.nl")
        .body(empty_body())
        .unwrap();
    let response = client.fetch(request).unwrap();
    let status = response.status().as_u16();
    let body = response.into_body();
    let body_str = from_utf8(&body).unwrap();

    println!("status: {status}");
    println!("body: {body_str}");
}
