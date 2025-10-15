#[cfg(feature = "server")]
use std::borrow::Cow;
#[cfg(feature = "server")]
use std::sync::LazyLock;

#[cfg(feature = "app-server")]
use reqwest::StatusCode;
#[cfg(feature = "server")]
use validator::ValidationError;

pub const COPYRIGHT: &str = "© 2025 Mango³ Group";
pub const PRIVACY_URL: &str = "https://mango3.app/#privacy";
pub const TERMS_URL: &str = "https://mango3.app/#terms";

#[cfg(feature = "server")]
pub static ERROR_ALREADY_EXISTS: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("already-exists").with_message(Cow::Borrowed("Already exists")));
#[cfg(feature = "server")]
pub static ERROR_IS_INVALID: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("invalid").with_message(Cow::Borrowed("Is invalid")));

pub const HEADER_APP_TOKEN: &str = "X-App-Token";
pub const HEADER_AUTHORIZATION: &str = "Authorization";

#[cfg(feature = "app-server")]
pub const RESPONSE_OK: (StatusCode, &str) = (StatusCode::OK, "\"Ok\"");
#[cfg(feature = "app-server")]
pub const RESPONSE_BAD_REQUEST: (StatusCode, &str) = (StatusCode::BAD_REQUEST, "\"Bad Request\"");
#[cfg(feature = "app-server")]
pub const RESPONSE_FORBIDDEN: (StatusCode, &str) = (StatusCode::FORBIDDEN, "\"Forbidden\"");
#[cfg(feature = "app-server")]
pub const RESPONSE_NOT_FOUND: (StatusCode, &str) = (StatusCode::NOT_FOUND, "\"Not Found\"");
#[cfg(feature = "app-server")]
pub const RESPONSE_UNAUTHORIZED: (StatusCode, &str) = (StatusCode::UNAUTHORIZED, "\"Unauthorized\"");
#[cfg(feature = "app-server")]
pub const RESPONSE_INTERNAL_SERVER_ERROR: (StatusCode, &str) =
    (StatusCode::INTERNAL_SERVER_ERROR, "\"Internal Server Error\"");
