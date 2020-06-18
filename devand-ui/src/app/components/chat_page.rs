use crate::app::services::ChatService;
use devand_core::chat::{ChatId, ChatMessage};
use devand_core::{PublicUserProfile, UserId};
use yew::{prelude::*, Properties};

pub struct ChatPage {
    props: Props,
    #[allow(dead_code)]
    service: ChatService,
    state: State,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub chat_with: String,
    pub me: PublicUserProfile,
}

pub enum Msg {
    AddMessages(Vec<ChatMessage>),
}

#[derive(Default)]
struct State {
    messages: Vec<ChatMessage>,
}

impl Component for ChatPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::AddMessages);

        let chat_id = ChatId::new(UserId(1), UserId(2)); // TPDP
        let mut service = ChatService::new(chat_id, callback);
        service.load_old_messages();
        let state = State::default();
        Self {
            props,
            service,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddMessages(messages) => {
                log::debug!("{:?}", messages);
                for msg in messages {
                    self.state.messages.push(msg);
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let messages = self.state.messages.iter().map(|msg| {
            let from_me = msg.from == self.props.me.id;
            let from_me_class = if from_me { "devand-from-me" } else { "devand-from-other" };
            html! {
                <div class=("devand-chat-message-bubble", from_me_class)>
                    <span class=("devand-chat-message-txt")>{ &msg.txt }</span>
                    <span class=("devand-timestamp")>{ format!("{:?}", msg.created_at) }</span>
                </div>
            }
        });
        html! {
            <>
                // <h1>{ format!("Chat with {}", self.props.chat_with.visible_name) }</h1>
                // <p>{ format!("WIP - chat with {} will be here", self.props.chat_with.visible_name) }</p>
                { for messages }
            </>
        }
    }
}
