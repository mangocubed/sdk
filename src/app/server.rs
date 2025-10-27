use std::net::SocketAddr;

use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use headers::authorization::{Bearer, Credentials};
use http::header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT};
use http::{HeaderMap, HeaderName, Method, StatusCode};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use validator::ValidationErrors;

use crate::constants::*;
use crate::core::config::APP_CONFIG;

use super::{ActionError, ActionSuccess};

pub fn extract_bearer(headers: &HeaderMap) -> Result<Bearer, (StatusCode, &'static str)> {
    let value = headers.get(HEADER_AUTHORIZATION).ok_or(RESPONSE_UNAUTHORIZED)?;

    Bearer::decode(value).ok_or(RESPONSE_UNAUTHORIZED)
}

pub async fn require_app_token<'a>(headers: &HeaderMap) -> Result<(), (StatusCode, &'a str)> {
    use crate::core::config::APP_CONFIG;

    let app_token = headers
        .get(HEADER_APP_TOKEN)
        .and_then(|value| value.to_str().ok())
        .ok_or(RESPONSE_FORBIDDEN)?
        .to_owned();

    if app_token == APP_CONFIG.token || APP_CONFIG.old_tokens.contains(&app_token) {
        Ok(())
    } else {
        Err(RESPONSE_FORBIDDEN)
    }
}

impl IntoResponse for ActionSuccess {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

impl ActionError {
    pub fn new(message: &str, validation_errors: Option<ValidationErrors>) -> Self {
        Self {
            message: message.to_owned(),
            code: 422,
            details: validation_errors,
        }
    }
}

impl IntoResponse for ActionError {
    fn into_response(self) -> Response {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({
                "message": self.message,
                "code": self.code,
                "details": self.details
            })),
        )
            .into_response()
    }
}

impl From<(StatusCode, &str)> for ActionError {
    fn from((status_code, message): (StatusCode, &str)) -> Self {
        Self {
            message: message.to_owned(),
            code: status_code.as_u16(),
            details: None,
        }
    }
}

pub async fn launch_request_server(router_fn: impl FnOnce(Router) -> Router) {
    let cors_layer = CorsLayer::new()
        .allow_headers([
            AUTHORIZATION,
            CONTENT_TYPE,
            USER_AGENT,
            HeaderName::from_static(HEADER_APP_TOKEN),
            HeaderName::from_static(HEADER_X_REAL_IP),
        ])
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_origin(Any);
    let trace_layer = TraceLayer::new_for_http();

    let router = router_fn(Router::new()).layer(cors_layer).layer(trace_layer);

    let address = &APP_CONFIG.request_address;

    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
