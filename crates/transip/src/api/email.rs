use std::str::FromStr;

use crate::{Client, Error, Result, client::Url};
use serde::{Deserialize, Serialize};
// use ureq::serde_json::Value;

const EMAIL_URL: &str = "email";
const MAIL_BOXES_URL: &str = "mailboxes";
const MAIL_FORWARDS_URL: &str = "mail-forwards";
// const MAIL_LISTS_URL: &str = "mail-lists";

trait UrlEmail {
    fn mailbox_domain(&self, domain_name: &str) -> String;
    fn mailbox_domain_item(&self, domain_name: &str, id: &str) -> String;
    fn mailforward_domain(&self, domain_name: &str) -> String;
    fn mailforward_domain_item(&self, domain_name: &str, id: &str) -> String;
    // fn maillist_domain(&self, domain_name: &str) -> String;
    // fn maillist_domain_item(&self, domain_name: &str, id: &str) -> String;
}

pub trait EmailApi {
    fn mailbox_delete(&mut self, domain_name: &str, id: &str) -> Result<()>;
    fn mailbox_insert(&mut self, domain_name: &str, mailbox: MailboxInsert) -> Result<()>;
    fn mailbox_item(&mut self, domain_name: &str, id: &str) -> Result<Mailbox>;
    fn mailbox_list(&mut self, domain_name: &str) -> Result<Vec<Mailbox>>;
    fn mailforward_delete(&mut self, domain_name: &str, id: &str) -> Result<()>;
    fn mailforward_insert(
        &mut self,
        domain_name: &str,
        mail_forward: MailForwardInsert,
    ) -> Result<()>;
    fn mailforward_item(&mut self, domain_name: &str, id: &str) -> Result<MailForward>;
    fn mailforward_list(&mut self, domain_name: &str) -> Result<Vec<MailForward>>;
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MailForward {
    pub id: u64,
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

#[derive(Debug, Deserialize, Serialize, PartialEq)]
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
    pub available_disk_space: u64,
    pub domain: String,
    pub forward_to: String,
    pub identifier: String,
    pub imap_port: u64,
    pub imap_server: String,
    pub is_locked: bool,
    pub local_part: String,
    pub pop3_port: u64,
    pub pop3_server: String,
    pub smtp_port: u64,
    pub smtp_server: String,
    pub status: String,
    pub used_disk_space: f32,
    pub webmail_url: String,
}

impl Default for Mailbox {
    fn default() -> Self {
        Self {
            identifier: String::default(),
            local_part: String::default(),
            domain: String::default(),
            forward_to: String::default(),
            available_disk_space: u64::default(),
            used_disk_space: f32::default(),
            status: "created".to_owned(),
            is_locked: false,
            imap_server: "imap.transip.email".to_owned(),
            imap_port: 993,
            smtp_server: "smtp.transip.email".to_owned(),
            smtp_port: 465,
            pop3_server: "pop3.transip.email".to_owned(),
            pop3_port: 995,
            webmail_url: "https://transip.email/".to_owned(),
        }
    }
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
        let mailbox_size_str = splitted
            .next()
            .ok_or(error(format!("Missing mailbox size on {}", s)))?;
        let mailbox_size = mailbox_size_str.parse::<u64>()?;

        if splitted.next().is_some() {
            Err(error(format!("Too many fields on {}", s)))
        } else {
            Ok(Self {
                local_part: username.to_owned(),
                max_disk_usage: mailbox_size,
                password: password.to_owned(),
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

    fn mailforward_domain_item(&self, domain_name: &str, id: &str) -> String {
        format!("{}/{}", self.mailforward_domain(domain_name), id)
    }

    // fn maillist_domain(&self, domain_name: &str) -> String {
    //     format!(
    //         "{}{}/{}/{}",
    //         self.prefix, EMAIL_URL, domain_name, MAIL_LISTS_URL
    //     )
    // }

    // fn maillist_domain_item(&self, domain_name: &str, id: &str) -> String {
    //     format!("{}/{}", self.maillist_domain(domain_name), id)
    // }
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

    fn mailforward_delete(&mut self, domain_name: &str, id: &str) -> Result<()> {
        self.delete_no_object(&self.url.mailforward_domain_item(domain_name, id))
    }

    fn mailforward_insert(
        &mut self,
        domain_name: &str,
        mail_forward: MailForwardInsert,
    ) -> Result<()> {
        self.post(&self.url.mailforward_domain(domain_name), mail_forward)
    }

    fn mailforward_item(&mut self, domain_name: &str, id: &str) -> Result<MailForward> {
        self.get::<MailForwardItem>(&self.url.mailforward_domain_item(domain_name, id))
            .map(|item| item.forward)
    }

    fn mailforward_list(&mut self, domain_name: &str) -> Result<Vec<MailForward>> {
        self.get::<MailForwardList>(&self.url.mailforward_domain(domain_name))
            .map(|list| list.forwards)
    }
}

#[cfg(not(target_family = "wasm"))]
#[cfg(test)]
mod test {
    const DEFAULT_CONTENT_TYPE: &str = "application/json";

    // use httpmock::{
    //     // matchers::{body_string, header, method, path}, Method::GET, Mock, ResponseTemplate
    // };

    use httpmock::Method::{DELETE, GET};

    use super::{Client, EmailApi, MailForward, Mailbox};

    const DOMAIN_NAME: &str = "transipdemo.be";

    fn mail_forward_for_transip_demo() -> MailForward {
        MailForward {
            id: 292883,
            local_part: "info".to_owned(),
            domain: DOMAIN_NAME.to_owned(),
            status: "created".to_owned(),
            forward_to: "info@paulmin.nl".to_owned(),
        }
    }

    fn mailbox_for_paulmin_demo() -> Mailbox {
        Mailbox {
            identifier: "info@paulmin.nl".to_owned(),
            local_part: "info".to_owned(),
            domain: "paulmin.nl".to_owned(),
            forward_to: "".to_owned(),
            available_disk_space: 2500,
            used_disk_space: 454.35,
            ..Default::default()
        }
    }

    #[test]
    fn mailbox_list() {
        let server = httpmock::MockServer::start();
        let relative_url = "/email/paulmin.nl/mailboxes";
        let body = r#"{"mailboxes":[{"identifier":"info@paulmin.nl","localPart":"info","domain":"paulmin.nl","forwardTo":"","availableDiskSpace":2500,"usedDiskSpace":454.35,"status":"created","isLocked":false,"imapServer":"imap.transip.email","imapPort":993,"smtpServer":"smtp.transip.email","smtpPort":465,"pop3Server":"pop3.transip.email","pop3Port":995,"webmailUrl":"https://transip.email/"}]}"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path(relative_url);
            then.status(200)
                .body(body)
                .header("Content-Type", DEFAULT_CONTENT_TYPE);
        });

        let mut client = Client::test(server.base_url());
        let mailbox_list = client.mailbox_list("paulmin.nl").unwrap();

        assert_eq!(mailbox_list, vec![mailbox_for_paulmin_demo()]);
        mock.assert_hits(1);
    }

    #[test]
    fn mailbox_item() {
        let server = httpmock::MockServer::start();
        let relative_url = "/email/paulmin.nl/mailboxes/info@paulmin.nl";
        let body = r#"{"mailbox":{"identifier":"info@paulmin.nl","localPart":"info","domain":"paulmin.nl","forwardTo":"","availableDiskSpace":2500,"usedDiskSpace":454.35,"status":"created","isLocked":false,"imapServer":"imap.transip.email","imapPort":993,"smtpServer":"smtp.transip.email","smtpPort":465,"pop3Server":"pop3.transip.email","pop3Port":995,"webmailUrl":"https://transip.email/"}}"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path(relative_url);
            then.status(200)
                .body(body)
                .header("Content-Type", DEFAULT_CONTENT_TYPE);
        });
        let mut client = Client::test(server.base_url());

        let item = client
            .mailbox_item("paulmin.nl", "info@paulmin.nl")
            .unwrap();

        assert_eq!(item, mailbox_for_paulmin_demo());
        mock.assert_hits(1);
    }

    #[test]
    fn mailbox_delete() {
        let server = httpmock::MockServer::start();
        let relative_url = "/email/paulmin.nl/mailboxes/info@paulmin.nl";

        let mock = server.mock(|when, then| {
            when.method(DELETE).path(relative_url);
            then.status(200);
        });

        let mut client = Client::test(server.base_url());
        client
            .mailbox_delete("paulmin.nl", "info@paulmin.nl")
            .unwrap();
        mock.assert_hits(1);
    }

    //     #[test]
    //     fn mailbox_insert() {
    //         let server = httpmock::MockServer::start();
    //         let body = r#"{"localPart":"test","maxDiskUsage":0,"password":"Ulkdjfi@kj#"}"#;
    //
    //         let mock = server.mock(|when, then| {
    //             when.method(POST)
    //                 .header("Content-Type", "application/json")
    //                 .body(body);
    //             then.status(201);
    //         });
    //
    //         let mut client = Client::test(server.base_url());
    //         let body_object = super::MailboxInsert {
    //             local_part: "test".to_owned(),
    //             max_disk_usage: 0,
    //             password: "Ulkdjfi@kj#".to_owned(),
    //         };
    //         client.mailbox_insert("paulmin.nl", body_object).unwrap();
    //         mock.assert_hits(1);
    //     }

    #[test]
    fn mailforward_list() {
        let server = httpmock::MockServer::start();
        let relative_url = "/email/transipdemo.be/mail-forwards";
        let body = r#"{"forwards":[{"id":292883,"localPart":"info","domain":"transipdemo.be","status":"created","forwardTo":"info@paulmin.nl"}]}"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path(relative_url);
            then.status(200)
                .body(body)
                .header("Content-Type", DEFAULT_CONTENT_TYPE);
        });

        let mut client = Client::test(server.base_url());
        let mailforward_list = client.mailforward_list(DOMAIN_NAME).unwrap();

        assert_eq!(mailforward_list, vec![mail_forward_for_transip_demo()]);
        mock.assert_hits(1);
    }

    #[test]
    fn mailforward_item() {
        let server = httpmock::MockServer::start();
        let relative_url = "/email/transipdemo.be/mail-forwards/292883";
        // let name = "mail-forward-item";
        let body = r#"{"forward":{"id":292883,"localPart":"info","domain":"transipdemo.be","status":"created","forwardTo":"info@paulmin.nl"}}"#;

        let mock = server.mock(|when, then| {
            when.method(GET).path(relative_url);
            then.status(200)
                .body(body)
                .header("Content-Type", DEFAULT_CONTENT_TYPE);
        });
        // Mock::given(method("GET"))
        //     .and(path(relative_url))
        //     .respond_with(
        //         ResponseTemplate::new(200)
        //             .set_body_bytes(body)
        //             .insert_header("Content-Type", DEFAULT_CONTENT_TYPE),
        //     )
        //     .expect(1)
        //     .named(name)
        //     .mount(&server);

        let mut client = Client::test(server.url(""));

        let item = client.mailforward_item(DOMAIN_NAME, "292883").unwrap();

        assert_eq!(item, mail_forward_for_transip_demo(),);
        mock.assert_hits(1);
    }

    #[test]
    fn mailforward_delete() {
        let server = httpmock::MockServer::start();
        let relative_url = "/email/transipdemo.be/mail-forwards/292883";
        // let name = "mail-forward-delete";

        let mock = server.mock(|when, then| {
            when.method(DELETE).path(relative_url);
            then.status(200);
        });
        // Mock::given(method("DELETE"))
        //     .and(path(relative_url))
        //     .respond_with(ResponseTemplate::new(200))
        //     .expect(1)
        //     .named(name)
        //     .mount(&server);

        let mut client = Client::test(server.url(""));
        client.mailforward_delete(DOMAIN_NAME, "292883").unwrap();
        mock.assert_hits(1);
    }

    //     #[test]
    //     fn mailforward_insert() {
    //         let server = httpmock::MockServer::start();
    //         let relative_url = "/email/transipdemo.be/mail-forwards";
    //         // let name = "mail-forward-insert";
    //         let body = r#"{"localPart":"test","forwardTo":"info@paulmin.nl"}"#;
    //
    //         let mock = server.mock(|when, then| {
    //             when.method(POST)
    //                 .path(relative_url)
    //                 .header("Content-Type", DEFAULT_CONTENT_TYPE)
    //                 .body(body);
    //             then.status(201);
    //         });
    //         // Mock::given(method("POST"))
    //         //     .and(path(relative_url))
    //         //     .and(header("Content-Type", "application/json"))
    //         //     .and(body_string(body))
    //         //     .respond_with(ResponseTemplate::new(201))
    //         //     .expect(1)
    //         //     .named(name)
    //         //     .mount(&server);
    //
    //         let mut client = Client::test(server.url(""));
    //         let entry = MailForwardInsert {
    //             local_part: "test".to_owned(),
    //             forward_to: "info@paulmin.nl".to_owned(),
    //         };
    //         client.mailforward_insert(DOMAIN_NAME, entry).unwrap();
    //         mock.assert_hits(1);
    //     }
}
