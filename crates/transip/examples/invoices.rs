use std::error::Error;

use transip::{api::account::AccountApi, configuration_from_environment, Client};

mod variables;

fn main() -> Result<(), Box<dyn Error>> {
    variables::set_variables();

    let mut client = configuration_from_environment().and_then(Client::try_from)?;

    // let list = client.invoice_list()?;

    // list.into_iter().for_each(|invoice| {
    //     dbg!(invoice);
    // });
    let item = client.invoice("0000.2017.0001.2152")?;
    dbg!(item);

    Ok(())
}
