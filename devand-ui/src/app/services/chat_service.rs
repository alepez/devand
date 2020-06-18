use devand_core::chat::ChatMessage;
use yew::prelude::Callback;

type NewMessagesCallback = Callback<Vec<ChatMessage>>;

// Comment line below to compile with mock_http enabled, so checker can run
// #[cfg(not(feature = "mock_http"))]
mod http;

#[cfg(feature = "mock_http")]
mod mock;

#[cfg(feature = "mock_http")]
pub use self::mock::ChatService;

#[cfg(not(feature = "mock_http"))]
pub use self::http::ChatService;
