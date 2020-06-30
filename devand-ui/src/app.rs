mod components;
mod elements;
mod services;

use self::components::{
    AffinitiesPage, ChatPage, CodeNowPage, NotFoundPage, SchedulePage, SecuritySettingsPage,
    SettingsPage,
};
use self::elements::busy_indicator;
use self::services::UserService;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, Switch};

use devand_core::{PublicUserProfile, User};

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
    #[to = "/settings/password"]
    SecuritySettings,
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
    verifying_email: bool,
}

pub enum Msg {
    UserStore(User),
    UserFetchOk(User),
    UserFetchErr,
    VerifyEmail,
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
            Msg::VerifyEmail => {
                log::debug!("Verify address");
                self.user_service.verify_email();
                self.state.verifying_email = true;
                true
            }
        }
    }

    fn view(&self) -> VNode {
        if let Some(user) = &self.state.user {
            self.view_ok(user)
        } else {
            busy_indicator()
        }
    }
}

impl App {
    fn view_ok(&self, user: &User) -> VNode {
        html! {
            <>
            { view_menu() }
            { self.view_routes(&user) }
            </>
        }
    }

    fn view_routes(&self, user: &User) -> VNode {
        let on_settings_change = self.link.callback(Msg::UserStore);
        let on_verify_email = self.link.callback(|_| Msg::VerifyEmail);
        let pub_user_profile: PublicUserProfile = user.clone().into();
        let user = user.clone();
        let verifying_email = self.state.verifying_email;

        html! {
            <Router<AppRoute>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Settings=> html!{ <SettingsPage on_change=on_settings_change.clone() user=user.clone() on_verify_email=on_verify_email.clone() verifying_email=verifying_email /> },
                        AppRoute::Affinities=> html!{ <AffinitiesPage/> },
                        AppRoute::CodeNow=> html!{ <CodeNowPage/> },
                        AppRoute::Schedule=> html!{ <SchedulePage me=pub_user_profile.clone()/> },
                        AppRoute::Chat(username) => html!{ <ChatPage chat_with=username me=pub_user_profile.clone() />},
                        AppRoute::NotFound(Permissive(missed_route)) => html!{ <NotFoundPage missed_route=missed_route/>},
                        AppRoute::SecuritySettings => html!{ <SecuritySettingsPage /> },
                    }
                })
                redirect = Router::redirect(|route: Route| { AppRoute::NotFound(Permissive(Some(route.route))) })
            />
        }
    }
}

fn view_menu() -> VNode {
    html! {
    <ul class=("devand-menu")>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::Settings classes="pure-menu-link" >{ "Settings" }</RouterAnchor></li>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::Affinities classes="pure-menu-link" >{ "Affinities" }</RouterAnchor></li>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::CodeNow classes="pure-menu-link" >{ "Code Now" }</RouterAnchor></li>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::Schedule classes="pure-menu-link" >{ "Schedule" }</RouterAnchor></li>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::SecuritySettings classes="pure-menu-link" >{ "Security" }</RouterAnchor></li>
    </ul>
    }
}
