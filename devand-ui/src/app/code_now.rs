use crate::app::services::CodeNowService;
use devand_core::{CodeNow, Language, LanguagePreference, PublicUserProfile, UserAffinity};
use serde_derive::{Deserialize, Serialize};
use yew::{prelude::*, Properties};

#[derive(Serialize, Deserialize, Default)]
pub struct State {
    code_now: Option<CodeNow>,
}

pub enum Msg {
    CodeNowUsersFetchOk(CodeNow),
    CodeNowUsersFetchErr,
}

pub struct CodeNowPage {
    props: Props,
    state: State,
    #[allow(dead_code)]
    service: CodeNowService,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for CodeNowPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();

        let callback = link.callback(|result: Result<CodeNow, anyhow::Error>| {
            if let Ok(code_now) = result {
                Msg::CodeNowUsersFetchOk(code_now)
            } else {
                Msg::CodeNowUsersFetchErr
            }
        });

        let mut service = CodeNowService::new(callback);

        service.restore();

        Self {
            props,
            state,
            service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CodeNowUsersFetchOk(code_now) => {
                self.state.code_now = Some(code_now);
            }
            Msg::CodeNowUsersFetchErr => {
                log::error!("CodeNow fetch error");
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
                {
                if let Some(code_now) = &self.state.code_now {
                    self.view_code_now_users(code_now)
                } else {
                    self.view_loading()
                }
                }
        }
    }
}

impl CodeNowPage {
    fn view_code_now_users(&self, code_now: &CodeNow) -> Html {
        let CodeNow {
            all_users,
            current_user,
        } = code_now.clone(); // TODO Avoid cloning

        let users = all_users
            .into_iter()
            .filter(|u| u.username != code_now.current_user.username);

        let user = current_user.into();
        let mut affinities: Vec<_> = devand_core::calculate_affinities_2(&user, users).collect();
        affinities.sort_unstable_by_key(|x| x.affinity);

        html! {
            <table class="user-affinities">
            { for affinities.iter().rev().map(|a| self.view_affinity(a)) }
            </table>
        }
    }

    fn view_affinity(&self, affinity: &UserAffinity) -> Html {
        let UserAffinity { user, affinity } = affinity;
        let PublicUserProfile {
            username,
            visible_name,
            languages,
        } = user;

        html! {
            <tr class="user-affinity">
                <td class="username">{ username }</td>
                <td class="visible_name">{ visible_name }</td>
                <td class="affinity">{ affinity.to_string() }</td>
                <td class="languages"> { for languages.clone().into_iter().map(|lang| self.view_language(lang)) } </td>
            </tr>
        }
    }

    fn view_language(&self, lang: (Language, LanguagePreference)) -> Html {
        let (lang, preferences) = lang;
        html! {
            <div class="language-control-group ">
                <span class="language-tag">{ lang }</span>
                { self.view_language_level(preferences.level) }
                { self.view_language_priority(preferences.priority) }
            </div>
        }
    }

    // TODO This is and exact copy of the one in settings page
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

    // TODO This is and exact copy of the one in settings page
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

    fn view_loading(&self) -> Html {
        html! {
            <p>{ "Loading..."}</p>
        }
    }
}
