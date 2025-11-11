use std::collections::HashMap;

#[cfg(feature = "server")]
use std::fmt::{Display, Formatter};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::ValidationErrors;

#[cfg(feature = "server")]
use dioxus::fullstack::AsStatusCode;
#[cfg(target_family = "wasm")]
use gloo_net::Error;
#[cfg(not(target_family = "wasm"))]
use reqwest::Error;

#[cfg(all(feature = "auth-client", feature = "server"))]
use crate::core::config::AUTH_CLIENT_CONFIG;

pub mod components;
mod data_storage;
pub mod hooks;
pub mod icons;
mod request;

#[cfg(feature = "server")]
pub mod server;

pub use data_storage::*;
pub use request::*;

#[cfg(feature = "server")]
pub use server::*;

static SPINNER_UNITS: GlobalSignal<HashMap<String, bool>> = GlobalSignal::new(HashMap::new);

pub type ActionResult = Result<ActionSuccess, ActionError>;

pub type ServFnResult<T = Option<String>, E = Option<String>> = Result<T, ServFnError<E>>;

pub type ServerResult<T = Option<String>> = ServFnResult<T>;

pub type ActionError = ServFnError<ValidationErrors>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ServFnError<T> {
    pub message: String,
    pub code: u16,
    pub details: Option<T>,
}

#[cfg(feature = "server")]
impl<T> Display for ServFnError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(feature = "server")]
impl<T> AsStatusCode for ServFnError<T> {
    fn as_status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.code).expect("Could not get status code")
    }
}

impl<T> From<ServerFnError> for ServFnError<T> {
    fn from(error: ServerFnError) -> Self {
        match error {
            ServerFnError::ServerError { message, code, .. } => Self {
                message,
                code,
                details: None,
            },
            _ => Self {
                message: error.to_string(),
                code: 500,
                details: None,
            },
        }
    }
}

#[cfg(feature = "server")]
impl<T> From<HttpError> for ServFnError<T> {
    fn from(error: HttpError) -> Self {
        Self {
            message: error.message.unwrap_or_else(|| error.status.to_string()),
            code: error.status.as_u16(),
            details: None,
        }
    }
}

#[cfg(feature = "server")]
impl<T> From<serde_json::Error> for ServFnError<T> {
    fn from(error: serde_json::Error) -> Self {
        Self {
            message: error.to_string(),
            code: 400,
            details: None,
        }
    }
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ActionSuccess {
    pub(crate) message: String,
    pub(crate) data: Value,
}

impl ActionSuccess {
    pub fn new(message: &str, data: Value) -> Self {
        Self {
            message: message.to_owned(),
            data,
        }
    }
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

pub fn launch(app: fn() -> Element) {
    #[cfg(not(feature = "server"))]
    {
        use crate::app::request::set_request_header;
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

#[cfg(feature = "auth-client")]
pub fn auth_client_provider_app_url() -> url::Url {
    #[cfg(feature = "server")]
    return AUTH_CLIENT_CONFIG.provider_app_url();

    #[cfg(not(feature = "server"))]
    env!("AUTH_CLIENT_PROVIDER_APP_URL")
        .parse()
        .expect("Could not parse Auth client provider app URL")
}

#[cfg(feature = "auth-client")]
pub fn auth_client_authorize_url() -> url::Url {
    let mut url = auth_client_provider_app_url().join("authorize").unwrap();

    #[cfg(feature = "server")]
    url.set_query(Some(&format!("client_id={}", AUTH_CLIENT_CONFIG.id())));

    #[cfg(not(feature = "server"))]
    url.set_query(Some(&format!("client_id={}", env!("AUTH_CLIENT_ID"))));

    url
}
