use devand_core::chat::Chats;
use devand_core::User;
use yew::prelude::Callback;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserServiceContent {
    pub user: User,
    pub chats: Chats,
}

type FetchCallback = Callback<Result<UserServiceContent, anyhow::Error>>;

// Comment line below to compile with mock_http enabled, so checker can run
#[cfg(not(feature = "mock_http"))]
mod http;

#[cfg(feature = "mock_http")]
mod mock;

#[cfg(feature = "mock_http")]
pub type UserService = self::mock::UserService;

#[cfg(not(feature = "mock_http"))]
pub type UserService = self::http::UserService;
