mod affinities;
mod code_now;
mod components;
mod languages;
mod not_found;
mod services;
mod settings;
mod style;

use self::affinities::AffinitiesPage;
use self::code_now::CodeNowPage;
use self::components::ChatPage;
use self::not_found::NotFoundPage;
use self::services::UserService;
use self::settings::SettingsPage;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, Switch};

use devand_core::User;

type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;
type RouterButton = yew_router::components::RouterButton<AppRoute>;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/affinities"]
    Affinities,
    #[to = "/code-now"]
    CodeNow,
    #[to = "/schedule"]
    Schedule,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
    #[to = "/dashboard"]
    Settings,
    #[to = "/chat/{username}"]
    Chat(String),
}

pub struct App {
    user_service: UserService,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Default)]
pub struct State {
    user: Option<User>,
    pending_save: bool,
}

pub enum Msg {
    UserStore(User),
    UserFetchOk(User),
    UserFetchErr,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_callback = link.callback(|result: Result<User, anyhow::Error>| match result {
            Ok(user) => Msg::UserFetchOk(user),
            Err(err) => {
                log::error!("{:?}", err);
                Msg::UserFetchErr
            }
        });

        let mut user_service = UserService::new(fetch_callback);
        user_service.restore();

        App {
            user_service,
            state: State::default(),
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UserStore(user) => {
                self.user_service.store(&user);
                false
            }
            Msg::UserFetchOk(user) => {
                log::debug!("User fetch ok");
                self.state.user = Some(user);
                self.state.pending_save = false;
                true
            }
            Msg::UserFetchErr => {
                log::error!("User fetch error");
                false
            }
        }
    }

    fn view(&self) -> VNode {
        let on_settings_change = self.link.callback(Msg::UserStore);
        let user = self.state.user.clone();

        html! {
            <>
            <div class=("pure-menu", "pure-menu-horizontal")>
                <ul class=("pure-menu-list")>
                    <li class=("pure-menu-item")><RouterAnchor route=AppRoute::Settings classes="pure-menu-link" >{ "Settings" }</RouterAnchor></li>
                    <li class=("pure-menu-item")><RouterAnchor route=AppRoute::Affinities classes="pure-menu-link" >{ "Affinities" }</RouterAnchor></li>
                    <li class=("pure-menu-item")><RouterAnchor route=AppRoute::CodeNow classes="pure-menu-link" >{ "Code Now" }</RouterAnchor></li>
                    <li class=("pure-menu-item")><RouterAnchor route=AppRoute::Schedule classes="pure-menu-link" >{ "Schedule" }</RouterAnchor></li>
                </ul>
            </div>
            <Router<AppRoute>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Settings=> html!{ <SettingsPage on_change=on_settings_change.clone() user=user.clone() /> },
                        AppRoute::Affinities=> html!{ <AffinitiesPage/> },
                        AppRoute::CodeNow=> html!{ <CodeNowPage/> },
                        AppRoute::Chat(username) => html!{ <ChatPage chat_with=username />},
                        AppRoute::NotFound(Permissive(missed_route)) => html!{ <NotFoundPage missed_route=missed_route/>},
                        _ => todo!()
                    }
                })
                redirect = Router::redirect(|route: Route| { AppRoute::NotFound(Permissive(Some(route.route))) })
            />
            </>
        }
    }
}
