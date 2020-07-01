use crate::signed_token;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailVerification {
    pub address: String,
}

impl signed_token::Signable for EmailVerification {
    const EXP_SECONDS: i64 = 3 * 60 * 60;
}
