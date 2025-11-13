#[cfg(feature = "server")]
use std::fmt::{Display, Formatter};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use validator::ValidationErrors;

#[cfg(feature = "server")]
use dioxus::fullstack::AsStatusCode;

pub type ActionResult = Result<ActionSuccess, ActionError>;

pub type ServFnResult<T = ()> = Result<T, HttpError>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ActionError {
    pub(crate) message: String,
    pub(crate) details: ValidationErrors,
}

#[cfg(feature = "server")]
impl ActionError {
    pub fn new(message: &str, details: Option<ValidationErrors>) -> Self {
        Self {
            message: message.to_owned(),
            details: details.unwrap_or_default(),
        }
    }

    pub fn err(message: &str, details: Option<ValidationErrors>) -> ActionResult {
        Err(Self::new(message, details))
    }
}

#[cfg(feature = "server")]
impl Display for ActionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<ServerFnError> for ActionError {
    fn from(error: ServerFnError) -> Self {
        match error {
            ServerFnError::ServerError { message, .. } => Self {
                message,
                details: ValidationErrors::default(),
            },
            _ => Self {
                message: error.to_string(),
                details: ValidationErrors::default(),
            },
        }
    }
}

#[cfg(feature = "server")]
impl From<HttpError> for ActionError {
    fn from(error: HttpError) -> Self {
        Self {
            message: error.message.unwrap_or_else(|| error.status.to_string()),
            details: ValidationErrors::default(),
        }
    }
}

#[cfg(feature = "server")]
impl From<serde_json::Error> for ActionError {
    fn from(error: serde_json::Error) -> Self {
        Self {
            message: error.to_string(),
            details: ValidationErrors::default(),
        }
    }
}

#[cfg(feature = "server")]
impl AsStatusCode for ActionError {
    fn as_status_code(&self) -> StatusCode {
        StatusCode::UNPROCESSABLE_ENTITY
    }
}

#[derive(Clone, Deserialize, PartialEq, Serialize)]
pub struct ActionSuccess {
    pub(crate) message: String,
    pub(crate) data: Value,
}

#[cfg(feature = "server")]
impl ActionSuccess {
    pub fn new(message: &str, data: Value) -> Self {
        Self {
            message: message.to_owned(),
            data,
        }
    }

    pub fn ok(message: &str, data: Value) -> ActionResult {
        Ok(Self::new(message, data))
    }
}
