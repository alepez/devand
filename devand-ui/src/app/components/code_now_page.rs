use crate::app::components::affinities_table::view_affinities_table;
use crate::app::elements::busy_indicator;
use crate::app::services::CodeNowService;
use crate::app::{AppRoute, RouterAnchor};
use devand_core::CodeNow;
use yew::{prelude::*, Properties};

#[derive(Default)]
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
        <>
        <h1>{ "Code Now" }</h1>
        {
            if let Some(code_now) = &self.state.code_now {
                view_code_now_users(code_now)
            } else {
                busy_indicator()
            }
        }
        </>
        }
    }
}

fn view_code_now_users(code_now: &CodeNow) -> Html {
    let CodeNow {
        all_users,
        current_user,
    } = code_now.clone();

    let users = all_users
        .into_iter()
        .filter(|u| u.username != code_now.current_user.username);

    let user = current_user.into();
    let mut affinities: Vec<_> = devand_core::calculate_affinities(&user, users).collect();
    affinities.sort_unstable_by_key(|x| x.affinity);

    if affinities.is_empty() {
        html! {
            <div class=("alert", "alert-warning")>
                {"Sorry, no matching online users found. You can try to "} <RouterAnchor route=AppRoute::Settings >{ "extend your languages selection." }</RouterAnchor>
            </div>
        }
    } else {
        html! {
        <>
            <p>{ "In the table below, you can see a list of online developers, sorted by analogy. Just click the chat icon to start chatting and organize your next pair-programming session." }</p>
            { view_affinities_table(&affinities) }
        </>
        }
    }
}
