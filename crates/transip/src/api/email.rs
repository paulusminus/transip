use std::str::FromStr;

use crate::{client::Url, Client, Error, Result};
use serde::{Deserialize, Serialize};

const EMAIL_URL: &str = "email";
const MAIL_BOXES_URL: &str = "mailboxes";
const MAIL_FORWARDS_URL: &str = "mail-forwards";
const MAIL_LISTS_URL: &str = "mail-lists";

trait UrlEmail {
    fn mailbox_domain(&self, domain_name: &str) -> String;
    fn mailbox_domain_item(&self, domain_name: &str, id: &str) -> String;
    fn mailforward_domain(&self, domain_name: &str) -> String;
    fn mailforward_domain_item(&self, domain_name: &str, id: u64) -> String;
    fn maillist_domain(&self, domain_name: &str) -> String;
    fn maillist_domain_item(&self, domain_name: &str, id: &str) -> String;
}

pub trait EmailApi {
    fn mailbox_delete(&mut self, domain_name: &str, id: &str) -> Result<()>;
    fn mailbox_insert(&mut self, domain_name: &str, mailbox: MailboxInsert) -> Result<()>;
    fn mailbox_item(&mut self, domain_name: &str, id: &str) -> Result<Mailbox>;
    fn mailbox_list(&mut self, domain_name: &str) -> Result<Vec<Mailbox>>;
    fn mailforward_delete(&mut self, domain_name: &str, id: u64) -> Result<()>;
    fn mailforward_insert(
        &mut self,
        domain_name: &str,
        mail_forward: MailForwardInsert,
    ) -> Result<()>;
    fn mailforward_item(&mut self, domain_name: &str, id: u64) -> Result<MailForward>;
    fn mailforward_list(&mut self, domain_name: &str) -> Result<Vec<MailForward>>;
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MailForward {
    pub id: u32,
    pub local_part: String,
    pub domain: String,
    pub status: String,
    pub forward_to: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailForwardItem {
    pub forward: MailForward,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailForwardList {
    pub forwards: Vec<MailForward>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailForwardInsert {
    pub local_part: String,
    pub forward_to: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailList {
    pub id: String,
    pub name: String,
    pub email_address: String,
    pub entries: Vec<String>,
}

impl FromStr for MailForwardInsert {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let error = Error::ParseMailForwardEntry;
        let mut splitted = s.split_ascii_whitespace();
        let username = splitted
            .next()
            .ok_or(error(format!("Missing username on {}", s)))?;
        let forward = splitted
            .next()
            .ok_or(error(format!("Missing forward address on {}", s)))?;
        if splitted.next().is_some() {
            Err(error(format!("Too many parameters on {}", s)))
        } else {
            Ok(Self {
                local_part: username.to_owned(),
                forward_to: forward.to_owned(),
            })
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailListList {
    mail_lists: Vec<MailList>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Mailbox {
    pub identifier: String,
    pub local_part: String,
    pub domain: String,
    pub forward_to: String,
    pub available_disk_space: String,
    pub used_disk_space: String,
    pub status: String,
    pub is_locked: String,
    pub imap_server: String,
    pub smtp_server: String,
    pub smtp_port: String,
    pub pop3_server: String,
    pub pop3_port: String,
    pub webmail_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxItem {
    pub mailbox: Mailbox,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxList {
    pub mailboxes: Vec<Mailbox>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxUpdate {
    pub max_disk_usage: u64,
    pub password: String,
    pub forward_to: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MailboxInsert {
    pub local_part: String,
    pub max_disk_usage: u64,
    pub password: String,
    pub forward_to: Option<String>,
}

impl FromStr for MailboxInsert {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let error = Error::ParseMailboxEntry;

        let mut splitted = s.split_ascii_whitespace();
        let username = splitted
            .next()
            .ok_or(error(format!("Missing username on {}", s)))?;
        let password = splitted
            .next()
            .ok_or(error(format!("Missing password on {}", s)))?;
        if splitted.next().is_some() {
            Err(error(format!("Too many fields on {}", s)))
        } else {
            Ok(Self {
                local_part: username.to_owned(),
                max_disk_usage: 0,
                password: password.to_owned(),
                forward_to: None,
            })
        }
    }
}

impl UrlEmail for Url {
    fn mailbox_domain(&self, domain_name: &str) -> String {
        format!(
            "{}{}/{}/{}",
            self.prefix, EMAIL_URL, domain_name, MAIL_BOXES_URL
        )
    }

    fn mailbox_domain_item(&self, domain_name: &str, id: &str) -> String {
        format!("{}/{}", self.mailbox_domain(domain_name), id)
    }

    fn mailforward_domain(&self, domain_name: &str) -> String {
        format!(
            "{}{}/{}/{}",
            self.prefix, EMAIL_URL, domain_name, MAIL_FORWARDS_URL
        )
    }

    fn mailforward_domain_item(&self, domain_name: &str, id: u64) -> String {
        format!("{}/{}", self.mailforward_domain(domain_name), id)
    }

    fn maillist_domain(&self, domain_name: &str) -> String {
        format!(
            "{}{}/{}/{}",
            self.prefix, EMAIL_URL, domain_name, MAIL_LISTS_URL
        )
    }

    fn maillist_domain_item(&self, domain_name: &str, id: &str) -> String {
        format!("{}/{}", self.maillist_domain(domain_name), id)
    }
}

impl EmailApi for Client {
    fn mailbox_delete(&mut self, domain_name: &str, id: &str) -> Result<()> {
        self.delete_no_object(&self.url.mailbox_domain_item(domain_name, id))
    }

    fn mailbox_insert(&mut self, domain_name: &str, mailbox: MailboxInsert) -> Result<()> {
        self.post(&self.url.mailbox_domain(domain_name), mailbox)
    }

    fn mailbox_item(&mut self, domain_name: &str, id: &str) -> Result<Mailbox> {
        self.get::<MailboxItem>(&self.url.mailbox_domain_item(domain_name, id))
            .map(|item| item.mailbox)
    }

    fn mailbox_list(&mut self, domain_name: &str) -> Result<Vec<Mailbox>> {
        self.get::<MailboxList>(&self.url.mailbox_domain(domain_name))
            .map(|list| list.mailboxes)
    }

    fn mailforward_delete(&mut self, domain_name: &str, id: u64) -> Result<()> {
        self.delete_no_object(&self.url.mailforward_domain_item(domain_name, id))
    }

    fn mailforward_insert(
        &mut self,
        domain_name: &str,
        mail_forward: MailForwardInsert,
    ) -> Result<()> {
        self.post(&self.url.mailforward_domain(domain_name), mail_forward)
    }

    fn mailforward_item(&mut self, domain_name: &str, id: u64) -> Result<MailForward> {
        self.get::<MailForwardItem>(&self.url.mailforward_domain_item(domain_name, id))
            .map(|item| item.forward)
    }

    fn mailforward_list(&mut self, domain_name: &str) -> Result<Vec<MailForward>> {
        self.get::<MailForwardList>(&self.url.mailforward_domain(domain_name))
            .map(|list| list.forwards)
    }
}

#[cfg(test)]
mod test {
    use crate::{api::email::MailForward, Client};

    use super::{EmailApi, MailForwardInsert};

    #[test]
    fn mailbox_list() {
        let mut client = Client::demo();
        let mailbox_list = client.mailbox_list("transipdemo.be").unwrap();
        assert_eq!(mailbox_list, vec![]);
    }

    #[test]
    fn mailforward_list() {
        let mut client = Client::demo();
        let mailforward_list = client.mailforward_list("transipdemo.be").unwrap();
        assert_eq!(
            mailforward_list,
            vec![MailForward {
                id: 19697,
                local_part: "".to_owned(),
                domain: "transipdemo.be".to_owned(),
                status: "created".to_owned(),
                forward_to: "feedback@transip.nl".to_owned(),
            }]
        );
    }

    #[test]
    fn mailforward_item() {
        let mut client = Client::demo();
        let item = client
            .mailforward_item("transipdemo.be", 19697)
            .unwrap();

        assert_eq!(
            item,
            MailForward {
                id: 19697,
                local_part: "".to_owned(),
                domain: "transipdemo.be".to_owned(),
                status: "created".to_owned(),
                forward_to: "feedback@transip.nl".to_owned(),
            }
        );
    }

    #[test]
    fn mailforward_delete() {
        let mut client = Client::demo();

        client.mailforward_delete("transipdemo.be", 19697).unwrap();
    }

    #[test]
    fn mailforward_insert() {
        let mut client = Client::demo();
        let entry = "info info@paulmin.nl".parse::<MailForwardInsert>().unwrap();
        client.mailforward_insert("transipdemo.be", entry).unwrap();
    }
}
