use transip_api::{default_account, ApiClient, TransipApiGeneral, WriteWrapper};

#[test]
fn main() {
    let account = default_account();
    assert!(account.is_ok());

    let (mut client, _): (ApiClient, WriteWrapper) = account.unwrap().into();
    assert_eq!(client.api_test().ok(), Some("pong".to_owned()));
}
