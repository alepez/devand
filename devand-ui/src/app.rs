mod affinities;
mod code_now;
mod languages;
mod not_found;
mod services;
mod settings;

use self::affinities::AffinitiesPage;
use self::code_now::CodeNowPage;
use self::not_found::NotFoundPage;
use self::settings::SettingsPage;

use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, Switch};

type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/affinities"]
    Affinities,
    #[to = "/code-now"]
    CodeNow,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
    #[to = "/dashboard"]
    Settings,
}

pub struct App {}

#[derive(Serialize, Deserialize, Default)]
pub struct State {}

pub enum Msg {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        App {}
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> VNode {
        html! {
            <>
            <div>
                <RouterAnchor route=AppRoute::Settings classes="pure-button" >{ "Settings" }</RouterAnchor>
                <RouterAnchor route=AppRoute::Affinities classes="pure-button" >{ "Affinities" }</RouterAnchor>
                <RouterAnchor route=AppRoute::CodeNow classes="pure-button" >{ "Code Now" }</RouterAnchor>
            </div>
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Settings=> html!{ <SettingsPage/> },
                        AppRoute::Affinities=> html!{ <AffinitiesPage/> },
                        AppRoute::CodeNow=> html!{ <CodeNowPage/> },
                        AppRoute::NotFound(Permissive(missed_route)) => html!{ <NotFoundPage missed_route=missed_route/>},
                    }
                })
                redirect = Router::redirect(|route: Route| { AppRoute::NotFound(Permissive(Some(route.route))) })
            />
            </>
        }
    }
}
