use yew::{prelude::*, Properties};

pub enum Msg {}

pub struct ChatPage {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub chat_with: String,
}

impl Component for ChatPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
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
            <p>{ format!("WIP - chat with {} will be here", self.props.chat_with) }</p>
        }
    }
}
