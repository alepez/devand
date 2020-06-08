use serde_derive::{Deserialize, Serialize};
use yew::{prelude::*, Properties};

#[derive(Serialize, Deserialize)]
pub struct State {}

pub enum Msg {}

pub struct NotFoundPage {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub missed_route: Option<String>,
}

impl Component for NotFoundPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        NotFoundPage { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
            {
                if let Some(missed_route) = &self.props.missed_route {
                    html! { <h1>{ format!("Page {} not found", missed_route) }</h1> }
                } else {
                    html! { <h1>{ "Page not found" }</h1> }
                }
            }
            </>
        }
    }
}
