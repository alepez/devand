#[derive(Debug)]
pub struct Email {
    pub recipient: String,
    pub subject: String,
    pub text: String,
    pub address_must_be_verified: bool,
}
