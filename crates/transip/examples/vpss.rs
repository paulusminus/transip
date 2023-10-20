use std::error::Error;

use transip::{Client, configuration_from_environment, api::vps::TransipApiVps};

#[allow(dead_code)]
const VPS2: &str = "paulusminus-vps2";
const VPS3: &str = "paulusminus-vps3";

mod variables;

#[allow(dead_code)]
fn print_list(client: &mut Client) -> Result<(), Box<dyn Error>> {
    let list = client.vps_list()?;
    for item in list {
        println!("Vps: {}", item.name);
    }
    Ok(())
}

fn print_vps(client: &mut Client, name: &str) -> Result<(), Box<dyn Error>> {
    let vps = client.vps(name)?;
    dbg!(vps);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    variables::set_variables();

    let mut client = configuration_from_environment().and_then(Client::try_from)?;

    print_vps(&mut client, VPS3)?;
    // client.vps_set_is_locked(VPS3, false)?;
    // client.vps_set_description(VPS3, "Hallo allemaal")?;

    Ok(())
}