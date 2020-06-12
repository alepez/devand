use devand_core::Affinity;
use devand_core::UserAffinity;
use yew::prelude::Callback;

type FetchCallback = Callback<Result<Vec<UserAffinity>, anyhow::Error>>;

// Compile even when mock_http is enabled, so checker can run
// #[cfg(not(feature = "mock_http"))]
// mod http;

#[cfg(feature = "mock_http")]
mod mock;

#[cfg(feature = "mock_http")]
pub type AffinitiesService = self::mock::AffinitiesService;

#[cfg(not(feature = "mock_http"))]
pub type AffinitiesService = self::http::AffinitiesService;
