use super::{FetchCallback, SecurityServiceContent};

pub struct SecurityService {
    callback: FetchCallback,
}

impl SecurityService {
    pub fn new(callback: FetchCallback) -> Self {
        Self { callback }
    }

    pub fn edit_password(&mut self, old_password: &str, new_password: &str) {
        let old_password_ok = old_password == "oldpassword";
        let new_password_ok = devand_core::auth::is_valid_password(new_password);

        if old_password_ok && new_password_ok {
            self.callback
                .emit(Ok(SecurityServiceContent::PasswordChanged))
        } else {
            self.callback
                .emit(Err(anyhow::anyhow!("Invalid password")))
        }
    }
    pub fn check_old_password(&mut self, old_password: &str) {
        let password_match = old_password == "oldpassword";
        self.callback
            .emit(Ok(SecurityServiceContent::OldPasswordCheck(password_match)))
    }
}
