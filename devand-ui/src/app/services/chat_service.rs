use devand_core::User;
use yew::prelude::Callback;

struct ChatHistory;
struct ChatMessage;

type HistoryLoadedCallback = Callback<ChatHistory>;
type NewMessageCallback = Callback<ChatMessage>;

// Comment line below to compile with mock_http enabled, so checker can run
// #[cfg(not(feature = "mock_http"))]
// mod http;

// #[cfg(feature = "mock_http")]
mod mock;

// #[cfg(feature = "mock_http")]
pub type ChatService = self::mock::ChatService;

// #[cfg(not(feature = "mock_http"))]
// pub type UserService = self::http::UserService;

