use std::collections::HashMap;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::ValidationErrors;

#[cfg(target_arch = "wasm32")]
use gloo_net::Error;
#[cfg(not(target_arch = "wasm32"))]
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

pub type ServerResult<T = Option<String>, E = Option<String>> = Result<T, ServerError<E>>;

pub type ActionError = ServerError<ValidationErrors>;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ServerError<T> {
    pub message: String,
    pub details: Option<T>,
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

impl<T> From<Error> for ServerError<T> {
    fn from(error: Error) -> Self {
        Self {
            message: error.to_string(),
            details: None,
        }
    }
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

pub async fn run_with_spinner<T, F>(id: &str, mut future: impl FnMut() -> F + 'static) -> T
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
