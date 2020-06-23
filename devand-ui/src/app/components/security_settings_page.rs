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
        html! {
        <>
        <h1>{ "Security" }</h1>
        <h2>{ "Password" }</h2>
        <div class="pure-form pure-form-stacked">
            <fieldset>
                <legend>{ "Profile" }</legend>
                <div class="pure-control-group">
                    <label for="old_password">{ "Old password:" }</label>
                    <input type="password" name="old_password" id="old_password" oninput=self.link.callback(move |e: InputData| Msg::SetOldPassword(e.value)) />
                </div>
                <div class="pure-control-group">
                    <label for="new_password">{ "New password:" }</label>
                    <input type="password" name="new_password" id="new_password" oninput=self.link.callback(move |e: InputData| Msg::SetNewPassword(e.value)) />
                </div>
                <div class="pure-control-group">
                    <label for="repeat_new_password">{ "Repeat new password:" }</label>
                    <input type="password" name="repeat_new_password" id="repeat_new_password" oninput=self.link.callback(move |e: InputData| Msg::SetRepeatNewPassword(e.value)) />
                    <span class="pure-form-message-inline">
                    {
                        if &self.state.new_password != &self.state.repeat_new_password {
                            "Password mismatch"
                        } else {
                            "Ok"
                        }
                    }
                    </span>
                </div>
                <button class="pure-button" onclick=self.link.callback(move |_| Msg::ChangePassword)>{ "Change password" }</button>
            </fieldset>
        </div>
        </>
        }
    }
}
