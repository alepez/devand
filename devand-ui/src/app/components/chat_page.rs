use crate::app::components::ChatInput;
use crate::app::services::ChatService;
use devand_core::chat::ChatMessage;
use devand_core::PublicUserProfile;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::{prelude::*, Properties};

pub struct ChatPage {
    props: Props,
    #[allow(dead_code)]
    service: ChatService,
    state: State,
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    poll_service: IntervalService,
    #[allow(dead_code)]
    poll_task: IntervalTask,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub chat_with: String,
    // TODO Having `me` as an Option is giving me troubles. Now we need it due to routes in app.rs
    pub me: Option<PublicUserProfile>,
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
}

impl Component for ChatPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let new_messages_callback = link.callback(Msg::AddMessages);
        let other_user_loaded_callback = link.callback(Msg::OtherUserLoaded);

        let mut service = ChatService::new(new_messages_callback, other_user_loaded_callback);

        //  When ChatPage is created, `props.me` may be some or none.
        //  We must start up the chat  (loading other user's profile and
        //  messages) only when `props.me` is some.
        if props.me.is_some() {
            service.load_other_user(&props.chat_with);
        }

        let state = State::default();

        let mut poll_service = IntervalService::new();

        let poll_task = poll_service.spawn(
            std::time::Duration::from_secs(1),
            link.callback(|_| Msg::Poll),
        );

        Self {
            props,
            service,
            state,
            link,
            poll_service,
            poll_task,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OtherUserLoaded(user) => {
                log::info!("Other user loaded");

                let me = self.props.me.as_ref().unwrap().id;

                if let Some(user) = &user {
                    let members = vec![user.id, me];
                    self.service.load_history(members);
                } else {
                    // TODO Display error
                }
                self.state.other_user = user;
                true
            }
            Msg::AddMessages(messages) => {
                log::debug!("{:?}", messages);
                for msg in messages {
                    self.state.messages.push(msg);
                }
                true
            }
            Msg::SendMessage(txt) => {
                log::debug!("{}", txt);
                self.service.send_message(txt);
                true
            }
            Msg::Poll => {
                let last_message = self.state.messages.last();
                self.service.poll(last_message);
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.me.is_none() && props.me.is_some() {
            self.service.load_other_user(&props.chat_with);
        }

        self.props = props;
        true
    }

    fn view(&self) -> Html {
        if let Some(me) = &self.props.me {
            self.view_main(me)
        } else {
            html! {}
        }
    }
}

impl ChatPage {
    fn view_main(&self, me: &PublicUserProfile) -> Html {
        let messages = self.state.messages.iter().map(|msg| {
            let from_me = msg.author == me.id;
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
        });
        html! {
            <>
                <h1>{ format!("Chat with {}", self.props.chat_with) }</h1>
                <p>{ format!("WIP - chat with {} will be here", self.props.chat_with) }</p>
                <div class="devand-chat-container">
                    <div class="devand-chat-messages">
                        { for messages }
                    </div>
                    <div class="devand-chat-footer">
                        <ChatInput on_return=self.link.callback(Msg::SendMessage) />
                    </div>
                </div>
            </>
        }
    }
}
