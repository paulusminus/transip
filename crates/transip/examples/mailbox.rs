use transip::api::email::{EmailApi, MailboxInsert};
use transip::{configuration_from_environment, Client, Result};
use ureq::serde_json::to_string_pretty;

const DOMAIN_NAME: &str = "paulmin.nl";
const EMAIL_ADDRESS: &str = "info@paulmin.nl";

fn main() -> Result<()> {
    env_logger::init();
    let mut client = configuration_from_environment().and_then(Client::try_from)?;
    let mailboxes = client.mailbox_list("paulmin.nl")?;
    println!("{}", to_string_pretty(&mailboxes)?);

    let mailbox = client.mailbox_item(DOMAIN_NAME, EMAIL_ADDRESS)?;
    println!("{:#?}", mailbox);

    let mail_forwarders = client.mailforward_list(DOMAIN_NAME)?;
    println!("{:#?}", mail_forwarders);

    let entry = "uiteraard uie#8373$KY 500".parse::<MailboxInsert>()?;
    println!("{}", to_string_pretty(&entry)?);
    // client.mailbox_insert(DOMAIN_NAME, entry)?;

    Ok(())
}
