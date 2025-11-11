use dioxus::fullstack::{get_request_headers, set_request_headers};
use dioxus::prelude::*;
use http::header::AUTHORIZATION;
use http::{HeaderName, HeaderValue};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

#[cfg(target_family = "wasm")]
use gloo_net::Error;
#[cfg(target_family = "wasm")]
use gloo_net::http::{Method, RequestBuilder};
#[cfg(not(target_family = "wasm"))]
use reqwest::{Error, Method, RequestBuilder};

#[cfg(target_family = "wasm")]
use crate::constants::HEADER_AUTHORIZATION;

pub struct Request {
    builder: RequestBuilder,
    json: Option<Value>,
}

impl Request {
    #[cfg(not(target_family = "wasm"))]
    fn new(method: Method, path: &str) -> Self {
        let client = reqwest::Client::new();

        let builder = client.request(method, request_url(path)).headers(get_request_headers());

        Self { builder, json: None }
    }

    #[cfg(target_family = "wasm")]
    pub fn new(method: Method, path: &str) -> Self {
        let mut builder = RequestBuilder::new(&request_url(path)).method(method);

        for (name, value) in get_request_headers().iter() {
            builder = builder.header(name.as_str(), value.to_str().unwrap());
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

    #[cfg(not(target_family = "wasm"))]
    pub fn query<T: Serialize + ?Sized>(mut self, query: &T) -> Self {
        self.builder = self.builder.query(query);

        self
    }

    #[cfg(target_family = "wasm")]
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
            #[cfg(not(target_family = "wasm"))]
            let builder = self.builder.json(&json);

            #[cfg(target_family = "wasm")]
            let builder = self.builder.json(&json)?;

            builder.send().await?
        } else {
            self.builder.send().await?
        };

        #[cfg(not(target_family = "wasm"))]
        let status_code = response.status().as_u16();

        #[cfg(target_family = "wasm")]
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

fn request_url(path: &str) -> String {
    format!("{}{}", env!("APP_REQUEST_URL"), path)
}
