use devand_core::schedule_matcher::AvailabilityMatch;
use yew::prelude::Callback;

type FetchCallback = Callback<Result<AvailabilityMatch, anyhow::Error>>;

// Comment line below to compile with mock_http enabled, so checker can run
// #[cfg(not(feature = "mock_http"))]
// mod http;

#[cfg(feature = "mock_http")]
mod mock;

#[cfg(feature = "mock_http")]
pub use self::mock::ScheduleService;

#[cfg(not(feature = "mock_http"))]
pub use self::http::ScheduleService;
