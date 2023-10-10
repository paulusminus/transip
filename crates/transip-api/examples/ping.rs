use transip_api::{configuration_from_environment, ApiClient, Result, TransipApiGeneral};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    
    let mut client: ApiClient = configuration_from_environment().and_then(ApiClient::try_from)?;
    assert_eq!(client.api_test().ok(), Some("pong".to_owned()));
    Ok(())
}
