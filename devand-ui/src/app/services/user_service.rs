use devand_core::User;
use yew::prelude::Callback;

type FetchCallback = Callback<Result<User, anyhow::Error>>;

#[cfg(not(feature = "mock_http"))]
mod http;

#[cfg(feature = "mock_http")]
mod mock;

#[cfg(feature = "mock_http")]
pub type UserService = self::mock::UserService;

#[cfg(not(feature = "mock_http"))]
pub type UserService = self::http::UserService;
