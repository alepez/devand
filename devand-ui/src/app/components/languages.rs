use devand_core::{Language, LanguagePreference, Priority};
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::callback::Callback;
use yew::{prelude::*, Properties};

pub struct State {
    language: Option<Language>,
    preferences: LanguagePreference,
}

pub enum Msg {
    ChangeLang(String),
    ChangeLevel(String),
    ChangePriority(String),
    Add,
    Nope,
}

pub struct AddLanguageComponent {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub on_add: Callback<(Language, LanguagePreference)>,
}

impl Component for AddLanguageComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State {
            language: None,
            preferences: LanguagePreference::default(),
        };

        AddLanguageComponent { props, state, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangeLang(s) => {
                let lang = Language::from_str(&s).ok();
                self.state.language = lang;
            }
            Msg::ChangeLevel(s) => {
                let level = devand_core::Level::from_str(&s).unwrap();
                self.state.preferences.level = level;
            }
            Msg::ChangePriority(s) => {
                let priority = Priority::from_str(&s).unwrap();
                self.state.preferences.priority = priority;
            }
            Msg::Add => {
                if let Some(lang) = self.state.language {
                    let lang_pref = (lang, self.state.preferences.clone());
                    self.props.on_add.emit(lang_pref);
                } else {
                    // Unreachable, because "Add" button is visible only when language is Some
                    unreachable!()
                }
            }
            Msg::Nope => {}
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="pure-g">
                { self.view_lang_select() }
                { self.view_level_select() }
                { self.view_priority_select() }
                { self.view_add_button() }
            </div>
        }
    }
}

impl AddLanguageComponent {
    fn view_add_button(&self) -> Html {
        let disabled = self.state.language.is_none();
        html! {
            <div class="pure-controls">
                <button class="pure-button" disabled=disabled onclick=self.on_add()>{ "Add" }</button>
            </div>
        }
    }

    fn view_lang_select(&self) -> Html {
        let selected_language = self.state.language;
        html! {
            <div class="pure-u-1 pure-u-sm-1-2">
            <label for="add_language">{ "Language: " }</label>
            <select name="add_language" id="add_language" class="pure-u-23-24" onchange=self.link.callback(move |cd: ChangeData| {
                if let ChangeData::Select(se) = cd {
                    Msg::ChangeLang(se.value())
                } else {
                    Msg::Nope
                }
            } )>
            <option value="" selected=(selected_language.is_none())></option>
            { for Language::iter().map(|x| {
               let selected = (selected_language == Some(x));
                html! {
                    <option value=x selected=selected>{ x }</option>
                }
            })
            }
            </select>
            </div>
        }
    }

    fn view_level_select(&self) -> Html {
        html! {
            <div class="pure-u-1 pure-u-sm-1-4">
            <label for="add_level">{ "Level: " }</label>
            <select name="add_level" id="add_level" class="pure-u-23-24" onchange=self.link.callback(move |cd: ChangeData| {
                if let ChangeData::Select(se) = cd {
                    Msg::ChangeLevel(se.value())
                } else {
                    Msg::Nope
                }
            } )>
            { for devand_core::Level::iter().map(|x| {
                html! {
                    <option value=x>{ x }</option>
                }
            })
            }
            </select>
            </div>
        }
    }

    fn view_priority_select(&self) -> Html {
        html! {
            <div class="pure-u-1 pure-u-sm-1-4">
            <label for="add_priority">{ "Priority: " }</label>
            <select name="add_priority" id="add_priority" class="pure-u-23-24" onchange=self.link.callback(move |cd: ChangeData| {
                if let ChangeData::Select(se) = cd {
                    Msg::ChangePriority(se.value())
                } else {
                    Msg::Nope
                }
            } )>
            { for Priority::iter().map(|x| {
                html! {
                    <option value=x>{ x }</option>
                }
            })
            }
            </select>
            </div>
        }
    }

    fn on_add(&self) -> Callback<yew::MouseEvent> {
        self.link.callback(|_| Msg::Add)
    }
}
