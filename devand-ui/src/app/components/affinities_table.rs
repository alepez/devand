use crate::app::components::user_affinity_bubble;
use crate::app::components::LanguageTag;
use devand_core::{Language, UserAffinity};
use std::str::FromStr;
use strum::IntoEnumIterator;
use yew::prelude::*;
use yewtil::NeqAssign;

pub struct AffinitiesTable {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub affinities: Vec<UserAffinity>,
}

pub enum Msg {
    FilterByLanguage(String),
    Nope,
}

#[derive(Default)]
struct State {
    filter_language: Option<Language>,
}

impl Component for AffinitiesTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();
        Self { props, state, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FilterByLanguage(lang) => {
                let filter_language = Language::from_str(&lang).ok();
                self.state.filter_language.neq_assign(filter_language)
            }
            Msg::Nope => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
            { self.view_filters() }
            { self.view_affinities_table() }
            </>
        }
    }
}

impl AffinitiesTable {
    fn view_filters(&self) -> Html {
        html! {
        <div class="devand-affinities-filters pure-form pure-form-stacked pure-g">
            { self.view_lang_select() }
        </div>
        }
    }

    fn view_lang_select(&self) -> Html {
        let selected_language = self.state.filter_language;

        let on_change = self.link.callback(move |cd: ChangeData| {
            if let ChangeData::Select(se) = cd {
                Msg::FilterByLanguage(se.value())
            } else {
                Msg::Nope
            }
        });

        let language_options = Language::iter()
            .map(|lang| (lang, selected_language == Some(lang)))
            .map(|(lang, selected)| {
                html! { <option value=lang.to_string() selected=selected>{ lang }</option> }
            });

        html! {
        <div class="pure-u-1 pure-u-sm-1-2">
            <label for="add_language">{ "Filter by Language" }</label>
            <select name="add_language" onchange=on_change class="pure-u-1">
                <option value="" selected=(selected_language.is_none())></option>
                { for language_options }
            </select>
        </div>
        }
    }

    fn view_affinities_table(&self) -> Html {
        let affinities = self
            .props
            .affinities
            .iter()
            .rev()
            .filter(|x| {
                dbg!(&self.state.filter_language);
                if let Some(filter_language) = self.state.filter_language {
                    x.user.languages.contains_key(&filter_language)
                } else {
                    true
                }
            })
            .map(|a| view_affinity(a));
        html! {
        <ul class="devand-user-affinities">
        { for affinities}
        </ul>
        }
    }
}

fn view_affinity(user_affinity: &UserAffinity) -> Html {
    let languages = user_affinity.user.languages.clone().into_sorted_vec();

    let languages_tags = languages.iter().map(|(lang, pref)| {
        html! { <LanguageTag lang=lang.clone() pref=pref.clone() /> }
    });

    html! {
    <li>{ user_affinity_bubble(user_affinity) } { for languages_tags }</li>
    }
}
