use devand_core::PublicUserProfile;
use yew::prelude::Callback;

type OtherUserLoadedCallback = Callback<Option<PublicUserProfile>>;

// Comment line below to compile with mock_http enabled, so checker can run
#[cfg(not(feature = "mock_http"))]
mod http;

#[cfg(feature = "mock_http")]
mod mock;

#[cfg(feature = "mock_http")]
pub use self::mock::UserProfileService;

#[cfg(not(feature = "mock_http"))]
pub use self::http::UserProfileService;
