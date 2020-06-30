use crate::app::components::AddLanguageComponent;
use crate::app::components::EditableLanguageTag;
use crate::app::elements::busy_indicator;
use devand_core::{Availability, Language, LanguagePreference, Languages, User};
use yew::{prelude::*, Properties};

use crate::app::components::AvailabilityTable;

pub enum Msg {
    UpdateVisibleName(String),
    ToggleVacationMode,
    AddLanguage((Language, LanguagePreference)),
    RemoveLanguage(Language),
    UpdateSchedule(Availability),
    VerifyAddress,
}

pub struct SettingsPage {
    props: Props,
    link: ComponentLink<Self>,
}

#[derive(Clone, Properties)]
pub struct Props {
    pub on_change: Callback<User>,
    pub user: Option<User>,
}

impl Component for SettingsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsPage { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddLanguage((lang, preferences)) => {
                self.update_user(move |user| {
                    user.settings
                        .languages
                        .insert(lang.clone(), preferences.clone());
                });
            }
            Msg::RemoveLanguage(lang) => {
                self.update_user(move |user| {
                    user.settings.languages.remove(&lang);
                });
            }
            Msg::UpdateVisibleName(s) => {
                self.update_user(move |user| {
                    user.visible_name = s;
                });
            }
            Msg::ToggleVacationMode => {
                self.update_user(move |user| {
                    user.settings.vacation_mode ^= true;
                });
            }
            Msg::UpdateSchedule(schedule) => {
                self.update_user(move |user| {
                    user.settings.schedule = schedule;
                });
            }
            Msg::VerifyAddress => {
                log::debug!("Verify address");
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="dashboard">
                <h1>{ "Settings" }</h1>
                {
                if let Some(user) = &self.props.user {
                    self.view_settings_panel(user)
                } else {
                    busy_indicator()
                }
                }
            </div>
        }
    }
}

fn view_vacation_mode_panel() -> Html {
    html! {
        <fieldset>
            <legend>{ "You are currently in vacation mode" }</legend>
        </fieldset>
    }
}

impl SettingsPage {
    fn view_settings_panel(&self, user: &User) -> Html {
        let settings = &user.settings;

        html! {
            <div class="pure-form pure-form-stacked">
                { self.view_profile_panel(user) }
                {
                    if user.settings.vacation_mode {
                        view_vacation_mode_panel()
                    } else {
                        self.view_availability_panel(&settings.schedule)
                    }
                }
                { self.view_languages_panel(&settings.languages) }
            </div>
        }
    }

    fn view_profile_panel(&self, user: &User) -> Html {
        html! {
            <fieldset>
                <legend>{ "Profile" }</legend>
                <div class="pure-control-group">
                    <label for="username">{ "Username:" }</label>
                    <input type="text" name="username" id="username" value=&user.username readonly=true />
                    <span class="pure-form-message-inline">{ "Username cannot be changed" }</span>
                </div>
                <div class="pure-control-group">
                    <label for="email">{ "Email:" }</label>
                    <input type="text" name="email" id="email" value=&user.email oninput=self.link.callback(move |e: InputData| Msg::UpdateVisibleName(e.value)) />
                    {
                    if !user.email_verified {
                        html! {
                            <span class="pure-form-message-inline">
                                <button
                                    class=("pure-button", "button-warning", "pure-button-primary")
                                    onclick=self.link.callback(|_| Msg::VerifyAddress)
                                    >{ "Verify" }
                                </button>
                                { " This address is not verified." }
                            </span>
                        }
                    } else {
                        html! {}
                    }
                    }
                </div>
                <div class="pure-control-group">
                    <label for="visible_name">{ "Visible name:" }</label>
                    <input type="text" name="visible_name" id="visible_name" value=&user.visible_name oninput=self.link.callback(move |e: InputData| Msg::UpdateVisibleName(e.value)) />
                </div>
                <div class="pure-control-group">
                    <label for="vacation_mode" class="pure-checkbox"><input type="checkbox" id="vacation_mode" checked=user.settings.vacation_mode onclick=self.link.callback(move |_| Msg::ToggleVacationMode) />{ " Vacation mode" }</label>
                </div>
            </fieldset>
        }
    }

    fn view_languages_panel(&self, languages: &Languages) -> Html {
        let languages_tags = languages.iter().map(|(&lang, pref)| {
            html! {
                <EditableLanguageTag lang=lang pref=pref on_remove=self.link.callback(move |_| Msg::RemoveLanguage(lang))/>
            }
        });

        html! {
            <fieldset>
                <div class="pure-g">
                    <legend class="pure-u-1">{ "Languages" }</legend>
                    <div class="pure-u-1">
                        { for languages_tags }
                    </div>
                    <div class="pure-u-1">
                        <AddLanguageComponent on_add=self.link.callback(move |lang_pref| Msg::AddLanguage(lang_pref))/>
                    </div>
                </div>
            </fieldset>
        }
    }

    fn view_availability_panel(&self, schedule: &Availability) -> Html {
        html! { <AvailabilityTable schedule=schedule on_change=self.link.callback(move |s: Availability| Msg::UpdateSchedule(s)) /> }
    }

    fn update_user<F>(&mut self, f: F)
    where
        F: FnOnce(&mut User),
    {
        if let Some(user) = &mut self.props.user {
            f(user);
            self.props.on_change.emit(user.clone());
        }
    }
}
