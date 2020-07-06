use crate::app::services::{SecurityService, SecurityServiceContent};
use yew::prelude::*;

pub struct SecuritySettingsPage {
    state: State,
    link: ComponentLink<Self>,
    service: SecurityService,
}

pub enum Msg {
    ServiceResponse(Result<SecurityServiceContent, anyhow::Error>),
    SetOldPassword(String),
    SetNewPassword(String),
    SetRepeatNewPassword(String),
    CheckOldPassword,
    ChangePassword,
}

#[derive(Default)]
struct State {
    old_password: String,
    new_password: String,
    repeat_new_password: String,
    old_password_ok: Option<bool>,
    password_changed: Option<bool>,
    generic_alert: Option<String>,
}

impl Component for SecuritySettingsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();
        let service_callback = link.callback(Msg::ServiceResponse);
        let service = SecurityService::new(service_callback);
        SecuritySettingsPage {
            link,
            state,
            service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetOldPassword(s) => {
                self.state.old_password = s;
                self.state.old_password_ok = None;
                self.state.generic_alert = None;
                true
            }
            Msg::SetNewPassword(s) => {
                self.state.new_password = s;
                self.state.generic_alert = None;
                true
            }
            Msg::SetRepeatNewPassword(s) => {
                self.state.repeat_new_password = s;
                true
            }
            Msg::CheckOldPassword => {
                if !self.state.old_password.is_empty() {
                    self.service.check_old_password(&self.state.old_password);
                }
                false
            }
            Msg::ChangePassword => {
                self.service
                    .edit_password(&self.state.old_password, &self.state.new_password);
                false
            }
            Msg::ServiceResponse(res) => {
                match res {
                    Ok(SecurityServiceContent::OldPasswordCheck(ok)) => {
                        self.state.old_password_ok = Some(ok);
                    }
                    Ok(SecurityServiceContent::PasswordChanged) => {
                        self.state.password_changed = Some(true);
                    }
                    Err(e) => {
                        self.state.generic_alert = Some(e.to_string());
                    }
                };
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let old_password_alert =
            check_old_password(&self.state.old_password, self.state.old_password_ok);

        let new_password_alert =
            check_new_password(&self.state.new_password, &self.state.repeat_new_password);

        let password_changed_alert = match self.state.password_changed {
            Some(true) => Ok("Password changed successfully."),
            Some(false) => Err("An error occurred while changing password. Please, retry."),
            None => Ok(""),
        };

        let submit_enabled = old_password_alert.is_ok()
            && new_password_alert.is_ok()
            && !self.state.old_password.is_empty()
            && !self.state.new_password.is_empty();

        html! {
        <>
        <h1>{ "Security" }</h1>
        <div class="pure-form pure-form-stacked">
            <fieldset>
                <legend>{ "Password" }</legend>

                <div class="pure-control-group">
                    <label for="old_password">{ "Old password:" }</label>
                    <input
                        type="password"
                        name="old_password"
                        id="old_password"
                        onblur=self.link.callback(|_| Msg::CheckOldPassword)
                        oninput=self.link.callback(|e: InputData| Msg::SetOldPassword(e.value)) />
                </div>

                <div class="pure-control-group">
                    <label for="new_password">{ "New password:" }</label>
                    <input
                        type="password"
                        name="new_password"
                        id="new_password"
                        oninput=self.link.callback(|e: InputData| Msg::SetNewPassword(e.value)) />
                </div>

                <div class="pure-control-group">
                    <label for="repeat_new_password">{ "Repeat new password:" }</label>
                    <input
                        type="password"
                        name="repeat_new_password"
                        id="repeat_new_password"
                        oninput=self.link.callback(|e: InputData| Msg::SetRepeatNewPassword(e.value)) />
                </div>

                { view_alert(old_password_alert) }
                { view_alert(new_password_alert) }

                <button
                    class="pure-button"
                    disabled=!submit_enabled
                    onclick=self.link.callback(|_| Msg::ChangePassword)>
                    { "Change password" }
                </button>

                { view_alert(password_changed_alert) }
                {
                    if let Some(msg) = &self.state.generic_alert {
                        html!{ <div class=("alert", "alert-danger")>{ msg }</div> }
                    } else {
                        html!{}
                    }
                }
            </fieldset>
        </div>
        </>
        }
    }
}

fn check_new_password(
    new_password: &str,
    repeat_new_password: &str,
) -> Result<&'static str, &'static str> {
    if new_password.is_empty() && repeat_new_password.is_empty() {
        return Ok("");
    }

    if !devand_core::auth::is_valid_password(new_password) {
        return Err("Password is too unsecure");
    }

    if new_password != repeat_new_password {
        return Err("Password mismatch");
    }

    Ok("New password ok")
}

fn check_old_password(
    old_password: &str,
    old_password_ok: Option<bool>,
) -> Result<&'static str, &'static str> {
    if old_password.is_empty() {
        Ok("")
    } else if old_password_ok == Some(false) {
        Err("Old password is wrong")
    } else if old_password_ok == Some(true) {
        Ok("Old password ok")
    } else {
        Ok("")
    }
}

fn view_alert(msg: Result<&str, &str>) -> Html {
    match msg {
        Ok(msg) if !msg.is_empty() => html! { <div class=("alert", "alert-success")>{ msg }</div> },
        Err(msg) if !msg.is_empty() => html! { <div class=("alert", "alert-danger")>{ msg }</div> },
        _ => html! {},
    }
}
