pub use error::Error;
pub use api_client::{get_default_account, ApiClient};
pub use general::TransipApiGeneral;
pub use vps::TransipApiVps;
pub use domain::{DnsEntry, TransipApiDomain};

mod authentication;
mod api_client;
// mod dns_lookup;
mod domain;
mod error;
mod general;
mod url;
// mod token;
mod vps;

pub type Result<T> = std::result::Result<T, Error>;

const ACME_CHALLENGE: &str = "_acme-challenge";

#[allow(dead_code)]
fn is_acme_challenge(dns_entry: &DnsEntry) -> bool {
    dns_entry.entry_type.as_str() == "TXT" && dns_entry.name.as_str() == ACME_CHALLENGE
}
