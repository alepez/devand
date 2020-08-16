mod components;
mod elements;
mod workers;

use self::components::*;
use self::elements::busy_indicator;
use self::workers::{main_worker, main_worker::MainWorker};
use devand_core::{PublicUserProfile, User};
use yew::prelude::*;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, Switch};

type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;
type RouterButton = yew_router::components::RouterButton<AppRoute>;

#[derive(Switch, Clone)]
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
    #[to = "/chat"]
    Chats,
    #[to = "/u/{username}"]
    UserProfile(String),
}

pub struct App {
    main_worker: Box<dyn Bridge<MainWorker>>,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Default)]
pub struct State {
    user: Option<User>,
    pending_save: bool,
    verifying_email: bool,
    online_users: usize,
}

pub enum Msg {
    UserStore(User),
    VerifyEmail,

    MainWorkerRes(main_worker::Response),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut main_worker = MainWorker::bridge(link.callback(Msg::MainWorkerRes));
        main_worker.send(main_worker::Request::Init);

        App {
            main_worker,
            state: State::default(),
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MainWorkerRes(res) => self.handle_main_worker_res(res),

            Msg::UserStore(user) => {
                self.main_worker
                    .send(main_worker::Request::SaveSelfUser(Box::new(user)).lazy());
                false
            }

            Msg::VerifyEmail => {
                log::debug!("Verify address");
                self.main_worker.send(main_worker::Request::VerifyEmail);
                self.state.verifying_email = true;
                true
            }
        }
    }

    fn view(&self) -> Html {
        if let Some(user) = &self.state.user {
            self.view_ok(user)
        } else {
            busy_indicator()
        }
    }
}

impl App {
    fn handle_main_worker_res(&mut self, res: main_worker::Response) -> bool {
        use main_worker::Response;

        match res {
            Response::SelfUserFetched(user) => {
                self.state.user = Some(*user);
                self.state.pending_save = false;
                true
            }

            Response::CodeNowFetched(code_now) => {
                let my_id = code_now.current_user.id;
                let online_users_now = code_now.all_users.iter().filter(|u| u.id != my_id).count();
                let changed = self.state.online_users != online_users_now;
                self.state.online_users = online_users_now;
                changed
            }

            Response::Error(err) => {
                log::error!("Error: {}", err);
                // TODO Show error alert
                false
            }

            _ => false,
        }
    }

    fn view_ok(&self, user: &User) -> Html {
        html! {
            <>
            { view_menu(user, self.state.online_users) }
            { self.view_routes(&user) }
            </>
        }
    }

    fn view_routes(&self, user: &User) -> Html {
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
                        AppRoute::Chats => html!{ <ChatsPage />},
                        AppRoute::NotFound(Permissive(missed_route)) => html!{ <NotFoundPage missed_route=missed_route/>},
                        AppRoute::SecuritySettings => html!{ <SecuritySettingsPage /> },
                        AppRoute::UserProfile(username) => html!{ <UserProfilePage username=username /> },
                    }
                })
                redirect = Router::redirect(|route: Route| { AppRoute::NotFound(Permissive(Some(route.route))) })
            />
        }
    }
}

fn view_menu(user: &User, online_users: usize) -> Html {
    html! {
    <ul class=("devand-menu")>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::Settings classes="pure-menu-link" >{ "Settings" }</RouterAnchor></li>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::Affinities classes="pure-menu-link" >{ "Affinities" }</RouterAnchor></li>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::CodeNow classes="pure-menu-link" >{ view_code_now(online_users) }</RouterAnchor></li>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::Schedule classes="pure-menu-link" >{ "Schedule" }</RouterAnchor></li>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::SecuritySettings classes="pure-menu-link" >{ "Security" }</RouterAnchor></li>
        <li class=("devand-menu-item")><RouterAnchor route=AppRoute::Chats classes="pure-menu-link" >{ view_messages(user.unread_messages) }</RouterAnchor></li>
    </ul>
    }
}

fn view_code_now(online_users: usize) -> Html {
    html! { <span>{ "Code Now"} <CountTag count=online_users /></span> }
}

fn view_messages(unread_messages: usize) -> Html {
    html! { <span>{ "Messages"} <CountTag count=unread_messages /></span> }
}
