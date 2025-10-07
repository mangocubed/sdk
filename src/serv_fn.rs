use std::collections::HashMap;

#[cfg(feature = "dioxus-server")]
use std::fmt::{Display, Formatter};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::ValidationErrors;

#[cfg(not(feature = "dioxus-server"))]
use dioxus::fullstack::client::Client;
#[cfg(feature = "dioxus-server")]
use dioxus::fullstack::mock_client::MockServerFnClient;
#[cfg(not(feature = "dioxus-server"))]
use futures::{Sink, Stream};
#[cfg(feature = "dioxus-server")]
use headers::authorization::Bearer;
#[cfg(feature = "dioxus-server")]
use http::{HeaderMap, StatusCode};
#[cfg(feature = "dioxus-web")]
use server_fn::client::browser::BrowserClient;
#[cfg(any(feature = "dioxus-desktop", feature = "dioxus-mobile"))]
use server_fn::client::reqwest::ReqwestClient;
#[cfg(not(feature = "dioxus-server"))]
use server_fn::error::FromServerFnError;
#[cfg(feature = "dioxus-web")]
use server_fn::request::browser::BrowserRequest;
#[cfg(feature = "dioxus-web")]
use server_fn::response::browser::BrowserResponse;

use crate::constants::HEADER_APP_TOKEN;

static SERV_FN_HEADERS: GlobalSignal<HashMap<String, String>> = GlobalSignal::new(HashMap::new);

pub fn remove_serv_fn_header(name: &str) {
    SERV_FN_HEADERS.write().remove(name);
}

pub fn set_serv_fn_header(name: &str, value: &str) {
    SERV_FN_HEADERS.write().insert(name.to_owned(), value.to_owned());
}

#[cfg(feature = "dioxus-server")]
pub type ServFnClient = MockServerFnClient;

#[cfg(not(feature = "dioxus-server"))]
pub struct ServFnClient;

#[cfg(feature = "dioxus-web")]
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
        let app_token = env!("APP_TOKEN");

        if !app_token.is_empty() {
            headers.append(HEADER_APP_TOKEN, app_token);
        }

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

#[cfg(any(feature = "dioxus-desktop", feature = "dioxus-mobile"))]
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
        let app_token = env!("APP_TOKEN");

        if !app_token.is_empty() {
            headers.append(HEADER_APP_TOKEN, app_token.parse().unwrap());
        }

        for (name, value) in SERV_FN_HEADERS() {
            let name: &'static str = Box::leak(name.into_boxed_str());

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
    InternalServer,
}

#[cfg(feature = "dioxus-server")]
impl Display for ServFnError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServFnError::Unauthorized => write!(f, "Unauthorized"),
            ServFnError::Forbidden => write!(f, "Forbidden"),
            ServFnError::NotFound => write!(f, "Not Found"),
            ServFnError::InternalServer => write!(f, "Internal Server"),
        }
    }
}

#[cfg(feature = "dioxus-server")]
fn set_serv_fn_status(status: StatusCode) {
    let server_context = dioxus::server::server_context();

    *server_context.status_mut() = status;
}

#[cfg(feature = "dioxus-server")]
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

    pub fn internal_server() -> Self {
        set_serv_fn_status(StatusCode::INTERNAL_SERVER_ERROR);

        Self::InternalServer
    }
}

#[cfg(feature = "dioxus-server")]
impl From<ServFnError> for ServerFnError<ServFnError> {
    fn from(value: ServFnError) -> Self {
        ServerFnError::ServerError(value)
    }
}

pub type ServFnResult<T = ()> = ServerFnResult<T, ServFnError>;

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct FormSuccess {
    pub(crate) message: String,
    pub(crate) data: Value,
}

#[cfg(feature = "dioxus-server")]
impl FormSuccess {
    pub fn new(message: &str, data: Value) -> Self {
        Self {
            message: message.to_owned(),
            data,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FormError {
    pub(crate) message: String,
    pub(crate) validation_errors: Option<ValidationErrors>,
}

#[cfg(feature = "dioxus-server")]
impl FormError {
    pub fn new(message: &str, validation_errors: Option<ValidationErrors>) -> Self {
        set_serv_fn_status(StatusCode::UNPROCESSABLE_ENTITY);

        Self {
            message: message.to_owned(),
            validation_errors,
        }
    }
}

#[cfg(feature = "dioxus-server")]
impl From<FormError> for ServerFnError<FormError> {
    fn from(value: FormError) -> Self {
        ServerFnError::ServerError(value)
    }
}

#[cfg(feature = "dioxus-server")]
impl From<ServerFnError<ServFnError>> for FormError {
    fn from(value: ServerFnError<ServFnError>) -> Self {
        Self {
            message: value.to_string(),
            validation_errors: None,
        }
    }
}

pub type FormResult = ServerFnResult<FormSuccess, FormError>;

#[cfg(feature = "dioxus-server")]
pub async fn extract_app_token() -> ServFnResult<Option<String>> {
    let app_token = extract_header(HEADER_APP_TOKEN).await?;

    Ok(app_token)
}

#[cfg(feature = "dioxus-server")]
pub async fn extract_bearer() -> ServFnResult<Option<Bearer>> {
    use axum_extra::TypedHeader;
    use headers::Authorization;

    if let Some(TypedHeader(Authorization(bearer))) = extract::<Option<TypedHeader<Authorization<Bearer>>>, _>()
        .await
        .map_err(|_| ServFnError::internal_server())?
    {
        Ok(Some(bearer))
    } else {
        Ok(None)
    }
}

#[cfg(feature = "dioxus-server")]
pub async fn extract_header(name: &str) -> ServFnResult<Option<String>> {
    let header_value = extract::<HeaderMap, _>()
        .await
        .map_err(|_| ServFnError::internal_server())?
        .get(name.to_lowercase())
        .and_then(|value| value.to_str().ok())
        .map(|token| token.to_owned());

    Ok(header_value)
}

#[cfg(feature = "dioxus-server")]
pub async fn require_app_token() -> ServFnResult<()> {
    let app_token = extract_app_token().await?;

    use crate::config::APP_CONFIG;

    if let Some(app_token) = app_token
        && (app_token == APP_CONFIG.token || APP_CONFIG.old_tokens.contains(&app_token.to_owned()))
    {
        Ok(())
    } else {
        Err(ServFnError::forbidden().into())
    }
}
