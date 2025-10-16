use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::ValidationErrors;

#[cfg(target_arch = "wasm32")]
use gloo_net::Error;
#[cfg(target_arch = "wasm32")]
use gloo_net::http::{Method, RequestBuilder};
#[cfg(not(target_arch = "wasm32"))]
use reqwest::{Error, Method, RequestBuilder};

use crate::constants::HEADER_APP_TOKEN;

#[cfg(target_arch = "wasm32")]
use crate::constants::HEADER_AUTHORIZATION;
#[cfg(all(feature = "auth-client", feature = "server"))]
use crate::core::config::AUTH_CLIENT_CONFIG;

pub mod components;
mod data_storage;
pub mod hooks;
pub mod icons;

#[cfg(feature = "server")]
pub mod server;

pub use data_storage::*;

static SPINNER_UNITS: GlobalSignal<HashMap<String, bool>> = GlobalSignal::new(HashMap::new);
static REQUEST_BEARER: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static REQUEST_HEADERS: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(Default::default);

pub type ActionResult = Result<ActionSuccess, ActionError>;

pub type ServerResult<T = (), E = ()> = Result<T, ServerError<E>>;

pub type ActionError = ServerError<ValidationErrors>;

#[derive(Clone, Deserialize, PartialEq)]
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

pub struct ClientRequest {
    builder: RequestBuilder,
    json: Option<Value>,
}

impl ClientRequest {
    #[cfg(not(target_arch = "wasm32"))]
    fn new(method: Method, path: &str) -> Self {
        let client = reqwest::Client::new();

        let mut builder = client
            .request(method, request_url(path))
            .header(HEADER_APP_TOKEN, env!("APP_TOKEN"));

        for (name, value) in request_headers() {
            builder = builder.header(name, value);
        }

        if let Some(token) = request_bearer() {
            builder = builder.bearer_auth(token);
        }

        Self { builder, json: None }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(method: Method, path: &str) -> Self {
        let mut builder = RequestBuilder::new(&request_url(path))
            .method(method)
            .header(HEADER_APP_TOKEN, env!("APP_TOKEN"));

        for (name, value) in request_headers() {
            builder = builder.header(&name, &value);
        }

        if let Some(token) = request_bearer() {
            builder = builder.header(HEADER_AUTHORIZATION, &format!("Bearer {token}"));
        }

        Self { builder, json: None }
    }

    pub fn delete(path: &str) -> Self {
        Self::new(Method::DELETE, path)
    }

    pub fn get(path: &str) -> Self {
        Self::new(Method::GET, path)
    }

    pub fn post(path: &str) -> Self {
        Self::new(Method::POST, path)
    }

    pub fn patch(path: &str) -> Self {
        Self::new(Method::PATCH, path)
    }

    pub fn json<T: Serialize + ?Sized>(mut self, json: &T) -> Self {
        self.json = Some(serde_json::to_value(json).unwrap());

        self
    }

    pub async fn send<T: DeserializeOwned, E: DeserializeOwned + From<Error>>(self) -> Result<T, E> {
        let response = if let Some(json) = self.json {
            #[cfg(not(target_arch = "wasm32"))]
            let builder = self.builder.json(&json);
            #[cfg(target_arch = "wasm32")]
            let builder = self.builder.json(&json)?;

            builder.send().await?
        } else {
            self.builder.send().await?
        };

        #[cfg(not(target_arch = "wasm32"))]
        let status_code = response.status().as_u16();
        #[cfg(target_arch = "wasm32")]
        let status_code = response.status();

        match status_code {
            200..=203 | 205..=299 => {
                let json = response.json().await?;

                Ok(json)
            }
            204 => Ok(serde_json::from_value(serde_json::Value::Null).unwrap()),
            _ => {
                let json = response.json().await?;

                Err(json)
            }
        }
    }
}

fn request_bearer() -> Option<String> {
    REQUEST_BEARER.lock().unwrap().clone()
}

fn request_headers() -> HashMap<String, String> {
    REQUEST_HEADERS.lock().unwrap().clone()
}

pub fn remove_request_bearer() {
    REQUEST_BEARER.lock().unwrap().take();
}

pub fn remove_request_header(name: &str) {
    REQUEST_HEADERS.lock().unwrap().remove(name);
}

pub fn set_request_bearer(token: &str) {
    REQUEST_BEARER.lock().unwrap().replace(token.to_owned());
}

pub fn set_request_header(name: &str, value: &str) {
    REQUEST_HEADERS
        .lock()
        .unwrap()
        .insert(name.to_owned(), value.to_owned());
}

fn request_url(path: &str) -> String {
    format!("{}{}", env!("APP_SERVER_URL"), path)
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
pub fn auth_client_provider_url() -> url::Url {
    #[cfg(feature = "server")]
    return AUTH_CLIENT_CONFIG.provider_url();

    #[cfg(not(feature = "server"))]
    env!("AUTH_CLIENT_PROVIDER_URL")
        .parse()
        .expect("Could not parse Auth client provider URL")
}

#[cfg(feature = "auth-client")]
pub fn auth_client_authorize_url() -> url::Url {
    let mut url = auth_client_provider_url().join("authorize").unwrap();

    #[cfg(feature = "server")]
    url.set_query(Some(&format!("client_id={}", AUTH_CLIENT_CONFIG.id())));

    #[cfg(not(feature = "server"))]
    url.set_query(Some(&format!("client_id={}", env!("AUTH_CLIENT_ID"))));

    url
}
