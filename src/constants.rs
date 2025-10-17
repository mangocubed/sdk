#[cfg(feature = "core")]
use std::borrow::Cow;
#[cfg(feature = "core")]
use std::sync::LazyLock;

#[cfg(feature = "server")]
use reqwest::StatusCode;
#[cfg(feature = "core")]
use validator::ValidationError;

pub const COPYRIGHT: &str = "© 2025 Mango³ Group";
pub const PRIVACY_URL: &str = "https://mango3.app/#privacy";
pub const TERMS_URL: &str = "https://mango3.app/#terms";

#[cfg(feature = "core")]
pub static ERROR_ALREADY_EXISTS: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("already-exists").with_message(Cow::Borrowed("Already exists")));
#[cfg(feature = "core")]
pub static ERROR_IS_INVALID: LazyLock<ValidationError> =
    LazyLock::new(|| ValidationError::new("invalid").with_message(Cow::Borrowed("Is invalid")));

#[cfg(feature = "app")]
pub const HEADER_APP_TOKEN: &str = "x-app-token";
#[cfg(feature = "app")]
pub const HEADER_AUTHORIZATION: &str = "authorization";
#[cfg(feature = "server")]
pub const HEADER_X_REAL_IP: &str = "x-real-ip";
#[cfg(feature = "server")]
pub const HEADER_USER_AGENT: &str = "user-agent";

#[cfg(feature = "server")]
pub const RESPONSE_OK: (StatusCode, &str) = (StatusCode::OK, "\"Ok\"");
#[cfg(feature = "server")]
pub const RESPONSE_BAD_REQUEST: (StatusCode, &str) = (StatusCode::BAD_REQUEST, "\"Bad Request\"");
#[cfg(feature = "server")]
pub const RESPONSE_FORBIDDEN: (StatusCode, &str) = (StatusCode::FORBIDDEN, "\"Forbidden\"");
#[cfg(feature = "server")]
pub const RESPONSE_NOT_FOUND: (StatusCode, &str) = (StatusCode::NOT_FOUND, "\"Not Found\"");
#[cfg(feature = "server")]
pub const RESPONSE_UNAUTHORIZED: (StatusCode, &str) = (StatusCode::UNAUTHORIZED, "\"Unauthorized\"");
#[cfg(feature = "server")]
pub const RESPONSE_INTERNAL_SERVER_ERROR: (StatusCode, &str) =
    (StatusCode::INTERNAL_SERVER_ERROR, "\"Internal Server Error\"");
