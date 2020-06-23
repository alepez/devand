use super::{FetchCallback, SecurityServiceContent};

pub struct SecurityService {
    callback: FetchCallback,
}

impl SecurityService {
    pub fn new(callback: FetchCallback) -> Self {
        Self { callback }
    }

    pub fn check_old_password(&mut self, old_password: &str) {
        let password_match = old_password == "oldpassword";
        log::debug!("old password ok: {}", password_match);
        self.callback
            .emit(Ok(SecurityServiceContent::OldPasswordCheck(password_match)))
    }
}
