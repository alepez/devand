use devand_core::CodeNow;
use yew::prelude::Callback;

type FetchCallback = Callback<Result<CodeNow, anyhow::Error>>;

// Comment line below to compile with mock_http enabled, so checker can run
#[cfg(not(feature = "mock_http"))]
mod http;

#[cfg(feature = "mock_http")]
mod mock;

#[cfg(feature = "mock_http")]
pub type CodeNowService = self::mock::CodeNowService;

#[cfg(not(feature = "mock_http"))]
pub type CodeNowService = self::http::CodeNowService;
