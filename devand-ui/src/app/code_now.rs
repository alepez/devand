use crate::app::components::LanguageTag;
use crate::app::services::CodeNowService;
use devand_core::{CodeNow, PublicUserProfile, UserAffinity};
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
                log::info!("{:?}", &code_now);
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
        if let Some(code_now) = &self.state.code_now {
            view_code_now_users(code_now)
        } else {
            view_loading()
        }
    }
}

fn view_code_now_users(code_now: &CodeNow) -> Html {
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
    let affinities = affinities
        .iter()
        .rev()
        .enumerate()
        .map(|(i, a)| view_affinity(a, i));

    if affinities.is_empty() {
        html! {
            <p> { "No matching users online" }</p>
        }
    } else {
        html! {
            <table class="user-affinities">
            { for affinities }
            </table>
        }
    }
}

fn view_affinity(affinity: &UserAffinity, i: usize) -> Html {
    let UserAffinity { user, affinity } = affinity;

    let PublicUserProfile {
        visible_name,
        languages,
        ..
    } = user;

    let languages = languages.clone().to_sorted_vec();

    let languages_tags = languages.iter().map(|(lang, pref)| {
        html! {
            <LanguageTag lang=lang pref=pref />
        }
    });

    let odd = if i % 2 == 1 {
        Some("pure-table-odd")
    } else {
        None
    };

    html! {
        <tr class=("user-affinity", odd)>
            <td class="affinity">{ affinity.to_string() }</td>
            <td class="visible_name">{ visible_name }</td>
            <td class="languages"> { for languages_tags } </td>
        </tr>
    }
}

fn view_loading() -> Html {
    html! {
        <p>{ "Loading..."}</p>
    }
}
