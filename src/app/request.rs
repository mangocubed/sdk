use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use dioxus::prelude::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

#[cfg(target_arch = "wasm32")]
use gloo_net::Error;
#[cfg(target_arch = "wasm32")]
use gloo_net::http::{Method, RequestBuilder};
#[cfg(not(target_arch = "wasm32"))]
use reqwest::{Error, Method, RequestBuilder};

use crate::constants::HEADER_APP_TOKEN;

#[cfg(target_arch = "wasm32")]
use crate::constants::HEADER_AUTHORIZATION;

static REQUEST_BEARER: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static REQUEST_HEADERS: LazyLock<Mutex<HashMap<String, String>>> = LazyLock::new(Default::default);

pub struct Request {
    builder: RequestBuilder,
    json: Option<Value>,
}

impl Request {
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.builder = self.builder.query(query);

        self
    }

    #[cfg(target_arch = "wasm32")]
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        let query = serde_json::to_value(query).unwrap();
        let query = query
            .as_object()
            .unwrap()
            .iter()
            .filter_map(|(k, v)| v.as_str().map(|v| (k.as_str(), v)));

        self.builder = self.builder.query(query);

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
    format!("{}{}", env!("APP_REQUEST_URL"), path)
}
