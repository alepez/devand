use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Email {
    pub recipient: String,
    pub subject: String,
    pub text: String,
    pub address_must_be_verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CcnEmail {
    pub recipients: Vec<String>,
    pub subject: String,
    pub text: String,
}
