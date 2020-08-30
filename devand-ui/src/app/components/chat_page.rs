use crate::app::components::{Alert, BusyIndicator, ChatInput};
use crate::app::workers::main_worker::Request::{
    ChatLoadHistory, ChatPoll, ChatSendMessage, LoadPublicUserProfileByUsername,
};
use crate::app::workers::{main_worker, main_worker::MainWorker};
use devand_core::chat::ChatMessage;
use devand_core::{PublicUserProfile, UserId};
use devand_text::Text;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::{prelude::*, Properties};

pub struct ChatPage {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    poll_task: IntervalTask,
    main_worker: Box<dyn Bridge<MainWorker>>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub chat_with: String,
    pub me: PublicUserProfile,
}

pub enum Msg {
    SendMessage(String),
    Poll,
    MainWorkerRes(main_worker::Response),
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
        let state = State::default();

        let poll_task = IntervalService::spawn(
            std::time::Duration::from_secs(5),
            link.callback(|_| Msg::Poll),
        );

        let mut main_worker = MainWorker::bridge(link.callback(Msg::MainWorkerRes));
        main_worker.send(LoadPublicUserProfileByUsername(props.chat_with.clone()));

        Self {
            props,
            state,
            link,
            poll_task,
            main_worker,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use main_worker::Response;

        match msg {
            Msg::SendMessage(txt) => {
                // TODO Make it work with multiple members chat
                if let Some(members) = self.two_members() {
                    self.main_worker.send(ChatSendMessage(members, txt));
                }
                false
            }

            Msg::Poll => {
                if !self.state.pending {
                    // TODO Make it work with multiple members chat
                    if let Some(members) = self.two_members() {
                        self.state.pending = true;
                        let from_created_at = self.state.messages.last().map(|x| x.created_at);
                        self.main_worker.send(ChatPoll(members, from_created_at));
                    }
                }
                false
            }

            Msg::MainWorkerRes(res) => match res {
                Response::PublicUserProfileFetched(other_user) => {
                    self.state.other_user = Some(*other_user);
                    // TODO Make it work with multiple members chat
                    if let Some(members) = self.two_members() {
                        self.main_worker.send(ChatLoadHistory(members));
                    }
                    true
                }

                Response::ChatHistoryLoaded(chat) => {
                    let devand_core::chat::ChatInfo {
                        mut messages,
                        members_info,
                    } = chat;

                    self.state.pending = false;
                    self.state.messages.append(&mut messages);

                    for member in members_info {
                        // TODO This works only for single member chats
                        self.state.verified_email = Some(member.verified_email);
                    }

                    true
                }

                Response::ChatNewMessagesLoaded(mut messages) => {
                    self.state.pending = false;
                    self.state.messages.append(&mut messages);
                    true
                }

                _ => false,
            },
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Changing of properties is not supported. This pages should always
        // be created from scratch by the router.
        false
    }

    fn view(&self) -> Html {
        if let Some(other_user) = &self.state.other_user {
            self.view_messages(other_user)
        } else {
            html! { <BusyIndicator /> }
        }
    }
}

impl ChatPage {
    // TODO ChatMembers should be a strong type, not a Vec
    // TODO Sorting when encoded to string should be enforced
    fn two_members(&self) -> Option<Vec<devand_core::UserId>> {
        let other_user = self.state.other_user.as_ref()?;
        let me = self.props.me.id;
        let other_user_id = other_user.id;
        let mut members = vec![other_user_id, me];
        members.sort();
        Some(members)
    }

    fn view_messages(&self, other_user: &PublicUserProfile) -> Html {
        let unverified_email = self.state.verified_email == Some(false);
        let msg_bubbles = self
            .state
            .messages
            .iter()
            .map(|msg| view_bubble(self.props.me.id, msg));

        html! {
            <>
                <h1>{ Text::ChatWith(&other_user.visible_name) }</h1>
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
                    <ChatInput on_return=self.link.callback(Msg::SendMessage) />
                </div>
            </>
        }
    }
}

fn view_unverified_email() -> Html {
    html! { <Alert>{ Text::UserWithUnverifiedEmail }</Alert> }
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
