use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use headers::HeaderMap;
use headers::authorization::{Bearer, Credentials};
use validator::ValidationErrors;

use crate::constants::{HEADER_APP_TOKEN, HEADER_AUTHORIZATION, RESPONSE_FORBIDDEN, RESPONSE_UNAUTHORIZED};

use super::{ActionError, ActionSuccess};

pub fn extract_bearer(headers: &HeaderMap) -> Result<Bearer, (StatusCode, &'static str)> {
    let value = headers.get(HEADER_AUTHORIZATION).ok_or(RESPONSE_UNAUTHORIZED)?;

    Bearer::decode(value).ok_or(RESPONSE_UNAUTHORIZED)
}

pub async fn require_app_token<'a>(headers: &HeaderMap) -> Result<(), (StatusCode, &'a str)> {
    use crate::config::APP_CONFIG;

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
                "details": self.details
            })),
        )
            .into_response()
    }
}

impl From<(StatusCode, &str)> for ActionError {
    fn from((_, message): (StatusCode, &str)) -> Self {
        Self {
            message: message.to_owned(),
            details: None,
        }
    }
}
