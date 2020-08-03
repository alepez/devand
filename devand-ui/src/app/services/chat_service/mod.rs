use devand_core::chat::ChatMessage;
use devand_core::{PublicUserProfile, UserChats};
use yew::prelude::Callback;

pub enum ChatServiceContent {
    OtherUser(PublicUserProfile),
    NewMessagess(Vec<ChatMessage>),
    AllChats(UserChats),
    OtherUserExtended(devand_core::chat::ChatMemberInfo),
}

// type NewMessagesCallback = Callback<Vec<ChatMessage>>;
// type OtherUserLoadedCallback = Callback<Option<PublicUserProfile>>;
type ChatServiceCallback = Callback<ChatServiceContent>;

// Comment line below to compile with mock_http enabled, so checker can run
#[cfg(not(feature = "mock_http"))]
mod http;

#[cfg(feature = "mock_http")]
mod mock;

#[cfg(feature = "mock_http")]
pub use self::mock::ChatService;

#[cfg(not(feature = "mock_http"))]
pub use self::http::ChatService;
