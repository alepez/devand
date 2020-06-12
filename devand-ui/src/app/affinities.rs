use devand_core::{Language, LanguagePreference, Languages, Schedule, User};
use serde_derive::{Deserialize, Serialize};
use yew::{prelude::*, Properties};

#[derive(Serialize, Deserialize, Default)]
pub struct State {}

pub enum Msg {}

pub struct AffinitiesPage {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for AffinitiesPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="affinities">
            </div>
        }
    }
}
