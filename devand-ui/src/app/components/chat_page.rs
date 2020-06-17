use yew::{prelude::*, Properties};

pub struct ChatPage {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub chat_with: String,
}

pub enum Msg {}

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
            <>
                <h1>{ format!("Chat with {}", self.props.chat_with) }</h1>
                <p>{ format!("WIP - chat with {} will be here", self.props.chat_with) }</p>
            </>
        }
    }
}
