use crate::app::components::ChatInput;
use crate::app::elements::busy_indicator;
use crate::app::services::ChatService;
use devand_core::chat::ChatMessage;
use devand_core::{PublicUserProfile, UserId};
use yew::services::interval::{IntervalService, IntervalTask};
use yew::{prelude::*, Properties};

pub struct ChatPage {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    service: ChatService,
    #[allow(dead_code)]
    poll_task: IntervalTask,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub chat_with: String,
    pub me: PublicUserProfile,
}

pub enum Msg {
    OtherUserLoaded(Option<PublicUserProfile>),
    AddMessages(Vec<ChatMessage>),
    SendMessage(String),
    Poll,
}

#[derive(Default)]
struct State {
    messages: Vec<ChatMessage>,
    other_user: Option<PublicUserProfile>,
    pending: bool,
}

impl Component for ChatPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let new_messages_callback = link.callback(Msg::AddMessages);
        let other_user_loaded_callback = link.callback(Msg::OtherUserLoaded);

        let mut service = ChatService::new(new_messages_callback, other_user_loaded_callback);

        service.load_other_user(&props.chat_with);

        let state = State::default();

        let poll_task = IntervalService::spawn(
            std::time::Duration::from_secs(1),
            link.callback(|_| Msg::Poll),
        );

        Self {
            props,
            service,
            state,
            link,
            poll_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OtherUserLoaded(user) => {
                if let Some(user) = &user {
                    let me = self.props.me.id;
                    let other = user.id;
                    let members = vec![other, me];
                    self.service.load_history(members);
                } else {
                    // TODO Display error
                }
                self.state.other_user = user;
                true
            }
            Msg::AddMessages(messages) => {
                self.state.pending = false;
                for msg in messages {
                    self.state.messages.push(msg);
                }
                true
            }
            Msg::SendMessage(txt) => {
                self.service.send_message(txt);
                false
            }
            Msg::Poll => {
                if !self.state.pending {
                    let last_message = self.state.messages.last();
                    self.state.pending = true;
                    self.service.poll(last_message);
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let mut changed = false;

        if self.props.chat_with != props.chat_with {
            self.service.load_other_user(&props.chat_with);
            changed = true;
        }

        if self.props.me.id != props.me.id {
            // Changing `me` does not make any sense
            unimplemented!()
        }

        self.props = props;
        changed
    }

    fn view(&self) -> Html {
        if let Some(other_user) = &self.state.other_user {
            self.view_messages(other_user)
        } else {
            busy_indicator()
        }
    }
}

impl ChatPage {
    fn view_messages(&self, other_user: &PublicUserProfile) -> Html {
        let msg_bubbles = self
            .state
            .messages
            .iter()
            .map(|msg| view_bubble(self.props.me.id, msg));

        html! {
            <>
                <h1>{ format!("Chat with {}", &other_user.visible_name) }</h1>
                <div class="devand-chat-container">
                    <div class="devand-chat-messages">
                        { for msg_bubbles }
                    </div>
                    <div class="devand-chat-footer">
                        <ChatInput on_return=self.link.callback(Msg::SendMessage) />
                    </div>
                </div>
            </>
        }
    }
}

fn view_bubble(me: UserId, msg: &ChatMessage) -> Html {
    let from_me = msg.author == me;
    let from_me_class = if from_me {
        "devand-from-me"
    } else {
        "devand-from-other"
    };
    html! {
        <div class=("devand-chat-message-bubble", from_me_class)>
            <span class=("devand-chat-message-txt")>{ &msg.txt }</span>
            <span class=("devand-timestamp")>{ format!("{:?}", msg.created_at) }</span>
        </div>
    }
}
