use crate::signed_token;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailVerification {
    pub address: String,
}

impl signed_token::Signable for EmailVerification {
    const EXP_SECONDS: i64 = 3 * 60 * 60;
}

#[derive(Clone)]
pub struct EmailVerificationToken(signed_token::SignedToken);

// TODO Use traits/derive/macro to automatically implement this
impl EmailVerificationToken {
    pub fn new(data: &EmailVerification, encoder: &signed_token::Encoder) -> Self {
        let token = encoder.encode(data).expect("Token is encoded");
        Self(token)
    }

    pub fn decode(&self, decoder: &signed_token::Decoder) -> Option<EmailVerification> {
        decoder.decode(&self.0)
    }
}

// TODO Use traits/derive/macro to automatically implement this
impl Into<String> for EmailVerificationToken {
    fn into(self) -> String {
        self.0.into()
    }
}

// TODO Use traits/derive/macro to automatically implement this
impl From<String> for EmailVerificationToken {
    fn from(s: String) -> Self {
        EmailVerificationToken(signed_token::SignedToken::from(s))
    }
}
