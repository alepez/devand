use crate::app::components::affinities_table::view_affinities_table;
use crate::app::components::common::{Alert, BusyIndicator};
use crate::app::workers::{main_worker, main_worker::MainWorker};
use crate::app::{AppRoute, RouterAnchor};
use devand_core::CodeNow;
use devand_text::Text;
use yew::{prelude::*, Properties};

#[derive(Default)]
pub struct State {
    code_now: Option<CodeNow>,
}

pub enum Msg {
    MainWorkerRes(main_worker::Response),
}

pub struct CodeNowPage {
    props: Props,
    state: State,
    _main_worker: Box<dyn Bridge<MainWorker>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for CodeNowPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();

        let mut main_worker = MainWorker::bridge(link.callback(Msg::MainWorkerRes));
        main_worker.send(main_worker::Request::LoadCodeNow);

        Self {
            props,
            state,
            _main_worker: main_worker,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MainWorkerRes(res) => {
                use main_worker::Response;

                match res {
                    Response::CodeNowFetched(code_now) => {
                        self.state.code_now = Some(*code_now);
                        true
                    }

                    _ => false,
                }
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
        <h1>{ Text::CodeNow }</h1>
        {
            if let Some(code_now) = &self.state.code_now {
                view_code_now_users(code_now)
            } else {
            html! { <BusyIndicator /> }
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
    let total_online_users_count = users.clone().count();
    let mut affinities: Vec<_> = devand_core::calculate_affinities(&user, users).collect();
    affinities.sort_unstable_by_key(|x| x.affinity);

    if affinities.is_empty() {
        if total_online_users_count > 0 {
            html! { <Alert> { Text::NoOnlineUsers } <RouterAnchor route=AppRoute::Settings >{ Text::ExtendYourLanguageSelection }</RouterAnchor> </Alert> }
        } else {
            html! { <Alert> { Text::NoMatchingOnlineUsers } <RouterAnchor route=AppRoute::Affinities >{ Text::ContactBestMatchingUsers }</RouterAnchor> </Alert> }
        }
    } else {
        html! {
        <>
            <p>{ Text::CodeNowTableDescription }</p>
            { view_affinities_table(&affinities) }
        </>
        }
    }
}
