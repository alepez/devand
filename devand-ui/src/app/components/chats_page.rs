use crate::app::elements::busy_indicator;
use crate::app::services::{ChatService, ChatServiceContent};
use crate::app::{AppRoute, RouterAnchor};
use devand_core::{UserChat, UserChats};
use yew::{prelude::*, Properties};
use yewtil::NeqAssign;

#[derive(Default)]
pub struct State {
    chats: Option<UserChats>,
}

pub enum Msg {
    ChatServiceContentFetched(ChatServiceContent),
}

pub struct ChatsPage {
    props: Props,
    state: State,
    #[allow(dead_code)]
    service: ChatService,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

impl Component for ChatsPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::default();

        let mut service = ChatService::new(link.callback(Msg::ChatServiceContentFetched));

        service.load_all_chats();

        Self {
            props,
            state,
            service,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChatServiceContentFetched(content) => match content {
                ChatServiceContent::AllChats(chats) => {
                    self.state.chats = Some(chats);
                    true
                }
                _ => false,
            },
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        html! {
            <>
                <h1>{ "Chats" }</h1>
                {
                if let Some(chats) = &self.state.chats {
                    view_chats(chats)
                } else {
                    busy_indicator()
                }
                }
            </>
        }
    }
}

fn view_chats(chats: &UserChats) -> Html {
    if chats.0.is_empty() {
        view_no_chats()
    } else {
        let chats = chats
            .0
            .iter()
            .rev()
            .filter(|c| !c.members.is_empty())
            .map(|c| view_chat(c));

        html! {
            <ul class="user-chats pure-table-striped">
            { for chats}
            </ul>
        }
    }
}

fn view_chat(chat: &UserChat) -> Html {
    match chat.members.len() {
        0 => html! { },
        1 => view_direct_chat(chat),
        _ => view_group_chat(chat),
    }
}

fn view_direct_chat(chat: &UserChat) -> Html {
    let other_user = &chat.members[0];
    let username = other_user.username.clone();
    let visible_name = other_user.visible_name.clone();
    let unread_messages = chat.unread_messages;

    html! {
    <li class=("user-chat")>
        <span class="visible_name"><RouterAnchor route=AppRoute::Chat(username) >{ visible_name }</RouterAnchor></span>
        {
            if unread_messages > 0 {
                html! { <span class="devand-messages-count">{ format!("{}", unread_messages) }</span> }
            } else {
                html! { }
            }
        }
    </li>
    }
}

fn view_group_chat(_chat: &UserChat) -> Html {
    todo!()
}

fn view_no_chats() -> Html {
    html! {
        <p>{ "You don't have any chat yet" }</p>
    }
}
