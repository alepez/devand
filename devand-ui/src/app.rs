mod settings;
mod not_found;
mod languages;
mod services;

use self::settings::SettingsPage;
use self::not_found::NotFoundPage;

use serde_derive::{Deserialize, Serialize};
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, Switch};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/"]
    Settings,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
}

pub struct App {
}

#[derive(Serialize, Deserialize, Default)]
pub struct State {}

pub enum Msg {
}

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
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    match switch {
                        AppRoute::Settings=> html!{ <SettingsPage/> },
                        AppRoute::NotFound(Permissive(missed_route)) => html!{ <NotFoundPage missed_route=missed_route/>},
                    }
                })
                redirect = Router::redirect(|route: Route| {
                    AppRoute::NotFound(Permissive(Some(route.route)))
                })
            />
        }
    }
}
