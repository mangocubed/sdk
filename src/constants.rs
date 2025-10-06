#[cfg(feature = "server")]
use std::borrow::Cow;
#[cfg(feature = "server")]
use std::sync::LazyLock;

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
