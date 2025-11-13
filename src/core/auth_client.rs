use std::borrow::Cow;
use std::sync::OnceLock;

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use super::config::AUTH_CLIENT_CONFIG;

static AUTH_CLIENT: OnceLock<AuthClient> = OnceLock::new();

#[derive(Clone)]
pub struct AuthClient<'a> {
    id: Uuid,
    secret: Cow<'a, str>,
    provider_api_url: Url,
}

impl<'a> Default for AuthClient<'a> {
    fn default() -> Self {
        Self {
            id: AUTH_CLIENT_CONFIG.id(),
            secret: Cow::Owned(AUTH_CLIENT_CONFIG.secret.clone()),
            provider_api_url: AUTH_CLIENT_CONFIG.provider_api_url(),
        }
    }
}

impl<'a> AuthClient<'a> {
    pub async fn refresh_auth(&self, auth: &Auth<'_>) -> anyhow::Result<Auth<'_>> {
        if auth.is_expired() {
            return Err(anyhow::anyhow!("Authorization is expired"));
        }

        let url = self.provider_api_url.join("private/refresh-auth")?;

        Ok(reqwest::Client::new()
            .post(url)
            .json(&serde_json::json!({
                "client_id": self.id,
                "client_secret": self.secret,
                "token": auth.token
            }))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn user_info(&self, auth: &Auth<'_>) -> anyhow::Result<UserInfo<'_>> {
        if auth.is_expired() {
            return Err(anyhow::anyhow!("Authorization is expired"));
        }

        let url = self.provider_api_url.join("private/user-info")?;

        Ok(reqwest::Client::new()
            .get(url)
            .bearer_auth(auth.token.clone())
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn verify_auth(&self, auth: &Auth<'_>) -> anyhow::Result<bool> {
        if auth.is_expired() {
            return Err(anyhow::anyhow!("Authorization is expired"));
        }

        let url = self.provider_api_url.join("private/verify-auth")?;

        Ok(reqwest::Client::new()
            .get(url)
            .bearer_auth(auth.token.clone())
            .send()
            .await?
            .status()
            .is_success())
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Auth<'a> {
    pub token: Cow<'a, str>,
    pub expires_at: DateTime<Utc>,
    pub refreshed_at: Option<DateTime<Utc>>,
}

impl<'a> Auth<'a> {
    pub fn new(token: &'a str, expires_at: DateTime<Utc>, refreshed_at: Option<DateTime<Utc>>) -> Self {
        Self {
            token: Cow::Borrowed(token),
            expires_at,
            refreshed_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }
}

#[derive(Deserialize)]
pub struct UserInfo<'a> {
    pub id: Uuid,
    pub username: Cow<'a, str>,
    pub email: Cow<'a, str>,
    pub display_name: Cow<'a, str>,
    pub initials: Cow<'a, str>,
    pub full_name: Cow<'a, str>,
    pub birthdate: NaiveDate,
    pub language_code: Cow<'a, str>,
    pub country_alpha2: Cow<'a, str>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub fn auth_client<'a>() -> &'a AuthClient<'a> {
    AUTH_CLIENT.get_or_init(AuthClient::default)
}
