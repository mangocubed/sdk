use std::collections::HashMap;
use std::io::Error;
use std::time::Duration;

use dioxus::fullstack::{get_request_headers, set_request_headers};
use dioxus::prelude::*;
use http::header::AUTHORIZATION;
use http::{HeaderName, HeaderValue};
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[cfg(all(feature = "identity-client", feature = "server"))]
use crate::core::config::IDENTITY_CLIENT_CONFIG;

pub mod components;
pub mod hooks;
pub mod icons;
pub mod serv_fn;
pub mod storage;

pub use serv_fn::*;

static SPINNER_UNITS: GlobalSignal<HashMap<String, bool>> = GlobalSignal::new(HashMap::new);

pub type ServerResult<T = Option<String>, E = Option<String>> = Result<T, ServFnError<E>>;

impl From<Error> for ActionError {
    fn from(error: Error) -> Self {
        Self {
            message: error.to_string(),
            details: ValidationErrors::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ServFnError<T> {
    pub message: String,
    pub code: u16,
    pub details: Option<T>,
}

impl<T> From<Error> for ServFnError<T> {
    fn from(error: Error) -> Self {
        Self {
            message: error.to_string(),
            code: 500,
            details: None,
        }
    }
}

pub fn get_request_bearer() -> Option<String> {
    get_request_headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(|value| value.to_owned())
}

pub fn remove_request_bearer() {
    remove_request_header(AUTHORIZATION);
}

pub fn remove_request_header(name: HeaderName) {
    let mut headers = get_request_headers();

    headers.remove(name);

    set_request_headers(headers);
}

pub fn set_request_bearer(token: &str) {
    set_request_header(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
}

pub fn set_request_header(name: HeaderName, value: HeaderValue) {
    let mut headers = get_request_headers();

    headers.insert(name, value);

    set_request_headers(headers);
}

pub fn launch(app: fn() -> Element) {
    #[cfg(not(feature = "server"))]
    {
        use crate::constants::X_APP_TOKEN;

        #[cfg(not(feature = "web"))]
        dioxus::fullstack::set_server_url(env!("APP_SERVER_URL"));

        set_request_header(X_APP_TOKEN, env!("APP_TOKEN").parse().unwrap());
    }

    dioxus::launch(app)
}

#[cfg(feature = "web")]
pub fn open_external_url(value: url::Url) {
    navigator().push(value.to_string());
}

#[cfg(feature = "desktop")]
pub fn open_external_url(value: url::Url) {
    let _ = dioxus::desktop::use_window().webview.load_url(value.as_ref());
}

#[cfg(feature = "mobile")]
pub fn open_external_url(value: url::Url) {
    let _ = dioxus::mobile::use_window().webview.load_url(value.as_ref());
}

#[cfg(feature = "server")]
pub fn open_external_url(_value: url::Url) {}

pub async fn run_with_spinner<T, F>(id: &str, future: impl Fn() -> F + 'static) -> T
where
    T: 'static,
    F: IntoFuture<Output = T> + 'static,
{
    SPINNER_UNITS.write().insert(id.to_owned(), true);

    let resp = future().await;

    SPINNER_UNITS.write().insert(id.to_owned(), false);

    resp
}

pub fn spinner_is_active() -> bool {
    SPINNER_UNITS.read().values().any(|&loading| loading)
}

#[cfg(feature = "identity-client")]
pub fn identity_provider_app_url() -> url::Url {
    #[cfg(feature = "server")]
    return IDENTITY_CLIENT_CONFIG.provider_app_url();

    #[cfg(not(feature = "server"))]
    env!("IDENTITY_CLIENT_PROVIDER_APP_URL")
        .parse()
        .expect("Could not parse Identity client provider app URL")
}

#[cfg(feature = "identity-client")]
pub fn identity_authorize_url() -> url::Url {
    let mut url = identity_provider_app_url().join("authorize").unwrap();

    #[cfg(feature = "server")]
    url.set_query(Some(&format!("client_id={}", IDENTITY_CLIENT_CONFIG.id())));

    #[cfg(not(feature = "server"))]
    url.set_query(Some(&format!("client_id={}", env!("IDENTITY_CLIENT_ID"))));

    url
}

pub async fn sleep(millis: u64) {
    let duration = Duration::from_millis(millis);

    #[cfg(not(target_family = "wasm"))]
    tokio::time::sleep(duration).await;

    #[cfg(target_family = "wasm")]
    gloo_timers::future::sleep(duration).await;
}
