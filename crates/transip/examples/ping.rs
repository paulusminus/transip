use transip::{api::general::GeneralApi, configuration_from_environment, Client, Result};

fn main() -> Result<()> {
    let mut client = configuration_from_environment().and_then(Client::try_from)?;
    let pong = client.api_test()?;
    println!("Received: {pong}");
    Ok(())
}
