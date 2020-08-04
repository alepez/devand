use crate::app::components::ChatInput;
use crate::app::elements::busy_indicator;
use crate::app::services::{ChatService, ChatServiceContent};
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
    ChatServiceContentFetched(ChatServiceContent),
    SendMessage(String),
    Poll,
}

#[derive(Default)]
struct State {
    messages: Vec<ChatMessage>,
    other_user: Option<PublicUserProfile>,
    pending: bool,
    verified_email: Option<bool>,
}

impl Component for ChatPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut service = ChatService::new(link.callback(Msg::ChatServiceContentFetched));

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
            Msg::ChatServiceContentFetched(content) => match content {
                ChatServiceContent::OtherUser(other_user) => {
                    let me = self.props.me.id;
                    let other_user_id = other_user.id;
                    let members = vec![other_user_id, me];
                    self.state.other_user = Some(other_user);
                    self.service.load_history(members);
                    true
                }
                ChatServiceContent::NewMessagess(new_messages) => {
                    self.state.pending = false;
                    for msg in new_messages {
                        self.state.messages.push(msg);
                    }
                    true
                }
                ChatServiceContent::OtherUserExtended(members_info) => {
                    self.state.verified_email = Some(members_info.verified_email);
                    true
                }
                _ => false,
            },
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
        let unverified_email = self.state.verified_email == Some(false);
        let msg_bubbles = self
            .state
            .messages
            .iter()
            .map(|msg| view_bubble(self.props.me.id, msg));

        html! {
            <>
                <h1>{ format!("Chat with {}", &other_user.visible_name) }</h1>
                {
                if unverified_email {
                    view_unverified_email()
                } else {
                    html! { }
                }
                }
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

fn view_unverified_email() -> Html {
    html! {
    <div class=("alert", "alert-warning")>
        { "This user may not receive email notification of this message (email not verified yet)" }
    </div>
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
            <span class=("devand-timestamp")>{ view_timestamp(&msg.created_at) }</span>
        </div>
    }
}

fn view_timestamp(t: &chrono::DateTime<chrono::Utc>) -> impl ToString {
    t.format("%B %d - %R UTC")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn timestamp_format() {
        use chrono::offset::TimeZone;
        let t = chrono::Utc.ymd(2020, 8, 2).and_hms_milli(20, 0, 1, 444);
        let formatted_t = view_timestamp(&t);
        assert_eq!("August 02 - 20:00 UTC", formatted_t.to_string());
    }
}
