use crate::signed_token;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordReset {
    pub user_id: i32,
}

impl signed_token::Signable for PasswordReset {
    const EXP_SECONDS: i64 = 3 * 60 * 60;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_decode() {
        use crate::Signable;
        let key = b"secret";
        let encoder = signed_token::Encoder::new_from_secret(key);
        let decoder = signed_token::Decoder::new_from_secret(key);

        let data = PasswordReset { user_id: 42 };
        let token = data.sign(&encoder);
        let token_string: String = token.clone().into();
        let decoded: PasswordReset = decoder.decode(&token).unwrap();
        assert_eq!(data.user_id, decoded.user_id);
        assert!(token_string.len() < 200);
    }
}
