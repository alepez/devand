mod schedule;

use crate::app::languages::AddLanguageComponent;
use crate::app::services::UserService;
use devand_core::{Language, LanguagePreference, Languages, Schedule, User};
use serde_derive::{Deserialize, Serialize};
use yew::{prelude::*, Properties};

use schedule::ScheduleTable;

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    user: Option<User>,
    pending_save: bool,
}

pub enum Msg {
    UpdateVisibleName(String),
    ToggleVacationMode,
    AddLanguage((Language, LanguagePreference)),
    RemoveLanguage(Language),
    UserFetchOk(User),
    UserFetchErr,
    UpdateSchedule(Schedule),
}

pub struct SettingsPage {
    props: Props,
    state: State,
    user_service: UserService,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for SettingsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_callback = link.callback(|result: Result<User, anyhow::Error>| {
            if let Ok(user) = result {
                Msg::UserFetchOk(user)
            } else {
                log::error!("{:?}", result);
                Msg::UserFetchErr
            }
        });

        let mut user_service = UserService::new(fetch_callback);

        user_service.restore();

        SettingsPage {
            props,
            state: State::default(),
            user_service,
            link,
        }
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
            Msg::UserFetchOk(user) => {
                self.state.user = Some(user);
                self.state.pending_save = false;
            }
            Msg::UserFetchErr => {
                log::error!("User fetch error");
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
                {
                if let Some(user) = &self.state.user {
                    self.view_settings_panel(user)
                } else {
                    self.view_loading()
                }
                }
            </div>
        }
    }
}

impl SettingsPage {
    fn view_loading(&self) -> Html {
        html! {
            <p>{ "Loading..."}</p>
        }
    }

    fn view_settings_panel(&self, user: &User) -> Html {
        let settings = &user.settings;

        html! {
            <div class="pure-form pure-form-stacked">
                { self.view_profile_panel(user) }
                {
                    if user.settings.vacation_mode {
                        self.view_vacation_mode_panel()
                    } else {
                        self.view_schedule_panel(&settings.schedule)
                    }
                }
                { self.view_languages_panel(&settings.languages) }
            </div>
        }
    }

    fn view_vacation_mode_panel(&self) -> Html {
        html! {
            <fieldset>
                <legend>{ "You are currently in vacation mode" }</legend>
            </fieldset>
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
                </div>
                <div class="pure-control-group">
                    <label for="visible_name">{ "Visible name:" }</label>
                    <input type="text" name="visible_name" id="visible_name" value=&user.visible_name oninput=self.link.callback(move |e: InputData| Msg::UpdateVisibleName(e.value)) />
                </div>
                <div class="pure-control-group">
                    <label for="vacation_mode" class="pure-checkbox"><input type="checkbox" id="vacation_mode" checked=user.settings.vacation_mode oninput=self.link.callback(move |e: InputData| Msg::ToggleVacationMode) />{ " Vacation mode" }</label>
                </div>
            </fieldset>
        }
    }

    fn view_languages_panel(&self, languages: &Languages) -> Html {
        html! {
            <fieldset>
                <div class="pure-g">
                    <legend class="pure-u-1">{ "Languages" }</legend>
                    { for languages.iter().map(|lang| self.view_language(lang)) }
                    <div class="pure-u-1">
                        <AddLanguageComponent on_add=self.link.callback(move |lang_pref| Msg::AddLanguage(lang_pref))/>
                    </div>
                </div>
            </fieldset>
        }
    }

    fn view_language(&self, lang: (&Language, &LanguagePreference)) -> Html {
        let (&lang, preferences) = lang;
        html! {
            <div class="language-control-group pure-u-1 pure-u-md-1-2 pure-u-lg-1-3">
                <span class="language-tag">
                    <button class="pure-button" onclick=self.link.callback(move |_| Msg::RemoveLanguage(lang))>{ "✖" }</button>
                    { lang }
                </span>
                { self.view_language_level(preferences.level) }
                { self.view_language_priority(preferences.priority) }
            </div>
        }
    }

    fn view_language_priority(&self, priority: devand_core::Priority) -> Html {
        let icon = match priority {
            devand_core::Priority::No => "X",
            devand_core::Priority::Low => ":|",
            devand_core::Priority::High => ":)",
        };
        let title = format!("{}", priority);
        let priority_class = format!("language-priority-tag-{}", priority).to_lowercase();
        let class = vec!["language-priority-tag", &priority_class];

        html! {
            <span class=class title=title>{ icon }</span>
        }
    }

    fn view_language_level(&self, level: devand_core::Level) -> Html {
        let stars = (1..=3).map(|x| x <= level.as_number());
        let icon = |on| if on { "★" } else { "☆" };
        let title = format!("{}", level);
        let level_class = format!("language-level-tag-{}", level).to_lowercase();
        let class = vec!["language-level-tag", &level_class];

        html! {
            <span class=class title=title>
                { for stars.map(|on| { html! { <span>{ icon(on) }</span> } }) }
            </span>
        }
    }

    fn view_schedule_panel(&self, schedule: &Schedule) -> Html {
        html! { <ScheduleTable schedule=schedule on_change=self.link.callback(move |s: Schedule| Msg::UpdateSchedule(s)) /> }
    }

    fn update_user<F>(&mut self, f: F)
    where
        F: FnOnce(&mut User),
    {
        if let Some(user) = &mut self.state.user {
            f(user);
            self.user_service.store(user);
        }
    }
}
