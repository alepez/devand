use super::{FetchCallback, SecurityServiceContent};
use devand_core::PasswordEdit;
use yew::format::Json;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

const API_PASSWORD_CHECK_URL: &'static str = "/api/password-check";
const API_PASSWORD_EDIT_URL: &'static str = "/api/password-edit";

pub struct SecurityService {
    callback: FetchCallback,
    service: FetchService,
    task: Option<FetchTask>,
}

impl SecurityService {
    pub fn new(callback: FetchCallback) -> Self {
        Self {
            callback,
            service: FetchService::new(),
            task: None,
        }
    }

    pub fn edit_password(&mut self, old_password: &str, new_password: &str) {
        log::debug!("Edit password (service)");
        let body = PasswordEdit {
            old_password: old_password.to_string(),
            new_password: new_password.to_string(),
        };
        let callback = self.callback.clone();
        let json = serde_json::to_string(&body).map_err(|_| anyhow::anyhow!("bo!"));
        let url = API_PASSWORD_EDIT_URL;
        let req = Request::post(url).body(json).unwrap();
        let handler = move |response: Response<Json<Result<(), anyhow::Error>>>| {
            let (meta, ..) = response.into_parts();
            if meta.status.is_success() {
                callback.emit(Ok(SecurityServiceContent::PasswordChanged))
            } else {
                callback.emit(Err(anyhow::anyhow!(meta.status)))
            }
        };

        self.task = self.service.fetch(req, handler.into()).ok();
    }

    pub fn check_old_password(&mut self, old_password: &str) {
        log::debug!("Check old password (service)");
        let body = PasswordEdit {
            old_password: old_password.to_string(),
            new_password: String::default(),
        };
        let callback = self.callback.clone();
        let json = serde_json::to_string(&body).map_err(|_| anyhow::anyhow!("bo!"));
        let url = API_PASSWORD_CHECK_URL;
        let req = Request::post(url).body(json).unwrap();
        let handler = move |response: Response<Json<Result<bool, anyhow::Error>>>| {
            let (meta, Json(data)) = response.into_parts();
            if let Ok(password_matches) = data {
                callback.emit(Ok(SecurityServiceContent::OldPasswordCheck(
                    password_matches,
                )))
            } else {
                callback.emit(Err(anyhow::anyhow!(meta.status)))
            }
        };

        self.task = self.service.fetch(req, handler.into()).ok();
    }
}
