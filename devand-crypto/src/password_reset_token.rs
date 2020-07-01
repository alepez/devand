use crate::signed_token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordReset {
    user_id: i32,
}

impl signed_token::Signable for PasswordReset {
    const EXP_SECONDS: i64 = 3600;
}

#[derive(Clone)]
pub struct PasswordResetToken(signed_token::SignedToken);

// TODO Use traits/derive/macro to automatically implement this
impl PasswordResetToken {
    pub fn new(data: &PasswordReset, encoder: &signed_token::Encoder) -> Self {
        let token = encoder.encode(data).expect("Token is encoded");
        Self(token)
    }

    pub fn decode(&self, decoder: &signed_token::Decoder) -> Option<PasswordReset> {
        decoder.decode(&self.0)
    }
}

// TODO Use traits/derive/macro to automatically implement this
impl Into<String> for PasswordResetToken {
    fn into(self) -> String {
        self.0.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_decode() {
        let key = b"secret";
        let encoder = signed_token::Encoder::new_from_secret(key);
        let decoder = signed_token::Decoder::new_from_secret(key);

        let data = PasswordReset { user_id: 42 };
        let token = PasswordResetToken::new(&data, &encoder);
        let token_string: String = token.clone().into();
        let decoded = token.decode(&decoder).unwrap();
        assert_eq!(data.user_id, decoded.user_id);
        assert!(token_string.len() < 200);
    }
}
