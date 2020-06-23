use crate::app::components::EditableLanguageTag;
use crate::app::elements::busy_indicator;
use crate::app::languages::AddLanguageComponent;
use devand_core::{Availability, Language, LanguagePreference, Languages, User};
use yew::{prelude::*, Properties};

use crate::app::components::AvailabilityTable;

pub struct SecuritySettingsPage {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties)]
pub struct Props {}

pub enum Msg {
    ChangePassword,
    SetOldPassword(String),
    SetNewPassword(String),
    SetRepeatNewPassword(String),
    CheckOldPassword,
}

#[derive(Default)]
struct State {
    old_password: String,
    new_password: String,
    repeat_new_password: String,
}

impl Component for SecuritySettingsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();
        SecuritySettingsPage { props, link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetOldPassword(s) => {
                self.state.old_password = s;
                // TODO Check old password is valid (needs access to db)
                true
            }
            Msg::SetNewPassword(s) => {
                self.state.new_password = s;
                // TODO Check if new password is valid (just pre-check length etc...)
                true
            }
            Msg::SetRepeatNewPassword(s) => {
                self.state.repeat_new_password = s;
                // TODO
                true
            }
            Msg::CheckOldPassword => {
                // TODO
                false
            }
            Msg::ChangePassword => {
                // TODO
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let new_password_feedback =
            check_new_password(&self.state.new_password, &self.state.repeat_new_password);

        let alert = new_password_feedback;

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

                { view_alert(alert) }

                <button
                    class="pure-button"
                    onclick=self.link.callback(|_| Msg::ChangePassword)>
                    { "Change password" }
                </button>
            </fieldset>
        </div>
        </>
        }
    }
}

fn check_new_password(new_password: &str, repeat_new_password: &str) -> Option<&'static str> {
    if new_password.is_empty() && repeat_new_password.is_empty() {
        return None;
    }

    if new_password != repeat_new_password {
        return Some("Password mismatch");
    }

    if !devand_core::auth::is_valid_password(new_password) {
        return Some("Password is too unsecure");
    }

    None
}

fn view_alert(msg: Option<&str>) -> Html {
    if let Some(msg) = msg {
        html!{ <div class=("alert", "alert-danger")>{ msg }</div> }
    } else {
        html!{  }
    }
}
