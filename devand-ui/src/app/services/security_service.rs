use yew::prelude::Callback;

pub enum SecurityServiceContent {
    OldPasswordCheck(bool),
}

type FetchCallback = Callback<Result<SecurityServiceContent, anyhow::Error>>;

// Comment line below to compile with mock_http enabled, so checker can run
// #[cfg(not(feature = "mock_http"))]
// mod http;

#[cfg(feature = "mock_http")]
mod mock;

#[cfg(feature = "mock_http")]
pub type SecurityService = self::mock::SecurityService;

// #[cfg(not(feature = "mock_http"))]
// pub type SecurityService = self::http::SecurityService;
