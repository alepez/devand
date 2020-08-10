use crate::app::components::AddLanguageComponent;
use crate::app::components::EditableLanguageTag;
use crate::app::elements::busy_indicator;
use devand_core::{
    Availability, Language, LanguagePreference, Languages, SpokenLanguage, SpokenLanguages, User,
};
use yew::{prelude::*, Properties};

use crate::app::components::AvailabilityTable;

pub enum Msg {
    UpdateVisibleName(String),
    UpdateBio(String),
    UpdateEmail(String),
    ToggleVacationMode,
    ToggleSpokenLanguage(SpokenLanguage),
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
    pub on_verify_email: Callback<()>,
    pub user: Option<User>,
    pub verifying_email: bool,
}

impl Component for SettingsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SettingsPage { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleSpokenLanguage(lang) => {
                self.update_user(move |user| {
                    let spoken_languages = &mut user.settings.spoken_languages;
                    let is_set = spoken_languages.0.contains(&lang);

                    if is_set {
                        spoken_languages.0.remove(&lang);
                    } else {
                        spoken_languages.0.insert(lang);
                    }
                });
            }
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
            Msg::UpdateBio(s) => {
                self.update_user(move |user| {
                    user.bio = s;
                });
            }
            Msg::UpdateEmail(s) => {
                self.update_user(move |user| {
                    // TODO Check if address is valid
                    user.email = s;
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
                self.props.on_verify_email.emit(());
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
            <div class="dashboard pure-g">
                <h1 class="pure-u-1">{ "Settings" }</h1>
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
        <fieldset class="pure-u-1">
            <legend>{ "You are currently in vacation mode" }</legend>
        </fieldset>
    }
}

impl SettingsPage {
    fn view_settings_panel(&self, user: &User) -> Html {
        let settings = &user.settings;

        html! {
            <form class="pure-form pure-form-stacked">
                { self.view_profile_panel(user) }
                {
                    if user.settings.vacation_mode {
                        view_vacation_mode_panel()
                    } else {
                        self.view_availability_panel(&settings.schedule)
                    }
                }
                { self.view_languages_panel(&settings.languages) }
                { self.view_spoken_languages_panel(&settings.spoken_languages) }
            </form>
        }
    }

    fn view_verify_email_button(&self, user: &User) -> Html {
        if !user.email_verified && !self.props.verifying_email {
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
        } else if !user.email_verified && self.props.verifying_email {
            html! {
                <span class=("pure-form-message-inline", "alert", "alert-success")>
                    { "Check your email for a link to verify your email address. If it doesn’t appear within a few minutes, check your spam folder." }
                </span>
            }
        } else {
            html! {}
        }
    }

    fn view_profile_panel(&self, user: &User) -> Html {
        html! {
            <fieldset class="pure-u-1 pure-u-md-1-2 pure-u-xl-1-4">
                <legend>{ "Profile" }</legend>
                <div class="pure-control-group">
                    <label for="username">{ "Username:" }</label>
                    <input type="text" name="username" id="username" class="pure-input-1" value=&user.username readonly=true />
                    <span class="pure-form-message-inline">{ "Username cannot be changed" }</span>
                </div>
                <div class="pure-control-group">
                    <label for="email">{ "Email:" }</label>
                    <input type="text" name="email" id="email" value=&user.email class="pure-input-1" oninput=self.link.callback(move |e: InputData| Msg::UpdateEmail(e.value)) />
                    { self.view_verify_email_button(user) }
                </div>
                <div class="pure-control-group">
                    <label for="visible_name">{ "Visible name:" }</label>
                    <input type="text" name="visible_name" id="visible_name" class="pure-input-1" value=&user.visible_name oninput=self.link.callback(move |e: InputData| Msg::UpdateVisibleName(e.value)) />
                </div>
                <div class="pure-control-group">
                    <label for="bio">{ "Bio:" }</label>
                    <textarea name="bio" class="pure-input-1" id="bio" value=&user.bio oninput=self.link.callback(move |e: InputData| Msg::UpdateBio(e.value)) />
                    <span class="pure-form-message-inline">{ "(max 160 characters)" }</span>
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
            <fieldset class="pure-u-1">
                <div class="pure-g">
                    <legend class="pure-u-1">{ "Languages" }</legend>
                    <div class="pure-u-1">
                    {
                        if !at_least_one_language_with_priority(languages) {
                            view_no_priority_warning()
                        } else {
                            html! {}
                        }
                    }
                    </div>
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

    fn view_spoken_languages_panel(&self, spoken_languages: &SpokenLanguages) -> Html {
        use strum::IntoEnumIterator;

        let options = SpokenLanguage::iter().map(|spoken_lang| {
            let checked = spoken_languages.contains(&spoken_lang);
            let input_id = format!("spoken-language-{}", spoken_lang);
            html! {
            <label for=input_id class="pure-checkbox">
                <input
                    type="checkbox"
                    id=input_id
                    value=spoken_lang
                    checked=checked
                    onclick=self.link.callback(move |_| Msg::ToggleSpokenLanguage(spoken_lang))
                    />
                { spoken_lang }
            </label>
            }
        });

        html! {
            <fieldset>
                <div class="pure-g">
                    <legend class="pure-u-1">{ "Spoken Languages" }</legend>
                    <div class="pure-u-1">
                    {
                        if spoken_languages.0.is_empty() {
                            view_no_spoken_language_warning()
                        } else {
                            html! {}
                        }
                    }
                    </div>
                    <div class="pure-u-1">
                        { for options }
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

fn view_no_priority_warning() -> Html {
    html! {
    <div class=("alert", "alert-warning")>
        { "Please, select at least one language with Low or High priority" }
    </div>
    }
}

fn view_no_spoken_language_warning() -> Html {
    html! {
    <div class=("alert", "alert-warning")>
        { "Please, select at least one spoken language" }
    </div>
    }
}

fn at_least_one_language_with_priority(languages: &Languages) -> bool {
    languages
        .iter()
        .any(|(_, pref)| pref.priority != devand_core::Priority::No)
}

#[cfg(test)]
mod test {
    use super::*;
    use devand_core::*;

    #[test]
    fn no_language_no_one_with_priority() {
        let languages = Languages::default();
        assert!(at_least_one_language_with_priority(&languages) == false);
    }

    #[test]
    fn some_languages_no_one_with_priority() {
        let mut languages = Languages::default();
        languages.insert(
            Language::Ada,
            LanguagePreference {
                level: Level::Novice,
                priority: Priority::No,
            },
        );
        assert!(at_least_one_language_with_priority(&languages) == false);
    }

    #[test]
    fn some_languages_someone_with_priority() {
        let mut languages = Languages::default();
        languages.insert(
            Language::Ada,
            LanguagePreference {
                level: Level::Novice,
                priority: Priority::High,
            },
        );
        assert!(at_least_one_language_with_priority(&languages));
    }
}
