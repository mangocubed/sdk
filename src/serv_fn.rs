use std::collections::HashMap;

#[cfg(feature = "server")]
use std::fmt::{Display, Formatter};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
use dioxus::fullstack::client::Client;
#[cfg(feature = "server")]
use dioxus::fullstack::mock_client::MockServerFnClient;
#[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
use futures::{Sink, Stream};
#[cfg(feature = "server")]
use http::{HeaderMap, HeaderValue, StatusCode};
#[cfg(feature = "web")]
use server_fn::client::browser::BrowserClient;
#[cfg(any(feature = "desktop", feature = "mobile"))]
use server_fn::client::reqwest::ReqwestClient;
#[cfg(any(feature = "web", feature = "desktop", feature = "mobile"))]
use server_fn::error::FromServerFnError;
#[cfg(feature = "web")]
use server_fn::request::browser::BrowserRequest;
#[cfg(feature = "web")]
use server_fn::response::browser::BrowserResponse;

static SERV_FN_HEADERS: GlobalSignal<HashMap<String, String>> = GlobalSignal::new(HashMap::new);

pub fn remove_serv_fn_header(name: &str) {
    SERV_FN_HEADERS.write().remove(name);
}

pub fn set_serv_fn_header(name: &str, value: &str) {
    SERV_FN_HEADERS.write().insert(name.to_owned(), value.to_owned());
}

#[cfg(feature = "server")]
pub type ServFnClient = MockServerFnClient;

#[cfg(not(feature = "server"))]
pub struct ServFnClient;

#[cfg(feature = "web")]
impl<E, IS, OS> Client<E, IS, OS> for ServFnClient
where
    E: FromServerFnError,
    IS: FromServerFnError,
    OS: FromServerFnError,
{
    type Request = BrowserRequest;
    type Response = BrowserResponse;

    fn send(req: Self::Request) -> impl Future<Output = Result<Self::Response, E>> + Send {
        let headers = req.headers();

        for (name, value) in SERV_FN_HEADERS() {
            headers.append(&name, &value);
        }

        <BrowserClient as Client<E, IS, OS>>::send(req)
    }

    fn open_websocket(
        path: &str,
    ) -> impl Future<
        Output = Result<
            (
                impl Stream<Item = Result<server_fn::Bytes, server_fn::Bytes>> + Send + 'static,
                impl Sink<server_fn::Bytes> + Send + 'static,
            ),
            E,
        >,
    > + Send {
        <BrowserClient as Client<E, IS, OS>>::open_websocket(path)
    }

    fn spawn(future: impl Future<Output = ()> + Send + 'static) {
        <BrowserClient as Client<E, IS, OS>>::spawn(future)
    }
}

#[cfg(any(feature = "desktop", feature = "mobile"))]
impl<E, IS, OS> Client<E, IS, OS> for ServFnClient
where
    E: FromServerFnError,
    IS: FromServerFnError,
    OS: FromServerFnError,
{
    type Request = reqwest::Request;
    type Response = reqwest::Response;

    fn send(mut req: Self::Request) -> impl Future<Output = Result<Self::Response, E>> + Send {
        let headers = req.headers_mut();

        for (name, value) in SERV_FN_HEADERS() {
            headers.append(name, value.parse().unwrap());
        }

        <ReqwestClient as Client<E, IS, OS>>::send(req)
    }

    fn open_websocket(
        path: &str,
    ) -> impl Future<
        Output = Result<
            (
                impl Stream<Item = Result<server_fn::Bytes, server_fn::Bytes>> + Send + 'static,
                impl Sink<server_fn::Bytes> + Send + 'static,
            ),
            E,
        >,
    > + Send {
        <ReqwestClient as Client<E, IS, OS>>::open_websocket(path)
    }

    fn spawn(future: impl Future<Output = ()> + Send + 'static) {
        <ReqwestClient as Client<E, IS, OS>>::spawn(future)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum ServFnError {
    Unauthorized,
    Forbidden,
    NotFound,
}

#[cfg(feature = "server")]
impl Display for ServFnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServFnError::Unauthorized => write!(f, "Unauthorized"),
            ServFnError::Forbidden => write!(f, "Forbidden"),
            ServFnError::NotFound => write!(f, "Not Found"),
        }
    }
}

#[cfg(feature = "server")]
fn set_serv_fn_status(status: StatusCode) {
    let server_context = dioxus::server::server_context();

    *server_context.status_mut() = status;
}

#[cfg(feature = "server")]
impl ServFnError {
    pub fn unauthorized() -> Self {
        set_serv_fn_status(StatusCode::UNAUTHORIZED);

        Self::Unauthorized
    }

    pub fn forbidden() -> Self {
        set_serv_fn_status(StatusCode::FORBIDDEN);

        Self::Forbidden
    }

    pub fn not_found() -> Self {
        set_serv_fn_status(StatusCode::NOT_FOUND);

        Self::NotFound
    }
}

#[cfg(feature = "server")]
impl From<ServFnError> for ServerFnError<ServFnError> {
    fn from(value: ServFnError) -> Self {
        ServerFnError::ServerError(value)
    }
}

pub type ServFnResult<T = ()> = ServerFnResult<T, ServFnError>;

#[derive(Clone, Deserialize, Serialize)]
pub struct FormSuccess {
    pub(crate) message: String,
}

#[cfg(feature = "server")]
impl FormSuccess {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormError {
    pub(crate) message: String,
    pub(crate) validation_errors: Option<ValidationErrors>,
}

#[cfg(feature = "server")]
impl FormError {
    pub fn new(message: &str, validation_errors: Option<ValidationErrors>) -> Self {
        set_serv_fn_status(StatusCode::UNPROCESSABLE_ENTITY);

        Self {
            message: message.to_owned(),
            validation_errors,
        }
    }
}

#[cfg(feature = "server")]
impl From<FormError> for ServerFnError<FormError> {
    fn from(value: FormError) -> Self {
        ServerFnError::ServerError(value)
    }
}

#[cfg(feature = "server")]
impl From<ServerFnError<ServFnError>> for FormError {
    fn from(value: ServerFnError<ServFnError>) -> Self {
        Self {
            message: value.to_string(),
            validation_errors: None,
        }
    }
}

pub type FormResult = ServerFnResult<FormSuccess, FormError>;

#[cfg(feature = "server")]
pub async fn extract_header_value(name: &str) -> ServFnResult<Option<HeaderValue>> {
    let header_value = extract::<HeaderMap, _>()
        .await
        .map_err(|error| ServFnError::forbidden())?
        .get(name.to_lowercase())
        .cloned();

    Ok(header_value)
}
