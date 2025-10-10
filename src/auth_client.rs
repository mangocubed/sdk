#[cfg(feature = "server")]
use std::borrow::Cow;
#[cfg(feature = "server")]
use std::collections::HashMap;

use url::Url;

#[cfg(feature = "server")]
use chrono::{DateTime, NaiveDate, Utc};
#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
use uuid::Uuid;

#[cfg(feature = "server")]
use crate::config::AUTH_CLIENT_CONFIG;

#[cfg(feature = "server")]
pub struct AuthClient<'a> {
    id: Uuid,
    secret: Cow<'a, str>,
    provider_url: Url,
}

#[cfg(feature = "server")]
impl<'a> Default for AuthClient<'a> {
    fn default() -> Self {
        Self {
            id: AUTH_CLIENT_CONFIG.id(),
            secret: Cow::Owned(AUTH_CLIENT_CONFIG.secret.clone()),
            provider_url: AUTH_CLIENT_CONFIG.provider_url(),
        }
    }
}

#[cfg(feature = "server")]
impl<'a> AuthClient<'a> {
    pub async fn refresh_auth(&self, auth: &Auth<'_>) -> anyhow::Result<Auth<'_>> {
        if auth.is_expired() {
            return Err(anyhow::anyhow!("Authorization is expired"));
        }

        let url = self.provider_url.join("priv-api/refresh-auth")?;
        let mut params = HashMap::new();

        params.insert("client_id", self.id.to_string());
        params.insert("client_secret", self.secret.to_string());
        params.insert("token", auth.token.to_string());

        Ok(reqwest::Client::new()
            .post(url)
            .form(&params)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn user_info(&self, auth: &Auth<'_>) -> anyhow::Result<UserInfo<'_>> {
        if auth.is_expired() {
            return Err(anyhow::anyhow!("Authorization is expired"));
        }

        let url = self.provider_url.join("priv-api/user-info")?;

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

        let url = self.provider_url.join("priv-api/verify-auth")?;

        Ok(reqwest::Client::new()
            .get(url)
            .bearer_auth(auth.token.clone())
            .send()
            .await?
            .status()
            .is_success())
    }
}

#[cfg(feature = "server")]
#[derive(Deserialize, Serialize)]
pub struct Auth<'a> {
    pub token: Cow<'a, str>,
    pub expires_at: DateTime<Utc>,
    pub refreshed_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
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

#[cfg(feature = "server")]
pub fn auth_client<'a>() -> AuthClient<'a> {
    AuthClient::default()
}

pub fn auth_client_provider_url() -> Url {
    #[cfg(feature = "server")]
    return AUTH_CLIENT_CONFIG.provider_url();

    #[cfg(not(feature = "server"))]
    env!("AUTH_CLIENT_PROVIDER_URL")
        .parse()
        .expect("Could not parse Auth client provider URL")
}

pub fn auth_client_authorize_url() -> Url {
    let mut url = auth_client_provider_url().join("authorize").unwrap();

    #[cfg(feature = "server")]
    url.set_query(Some(&format!("client_id={}", AUTH_CLIENT_CONFIG.id())));

    #[cfg(not(feature = "server"))]
    url.set_query(Some(&format!("client_id={}", env!("AUTH_CLIENT_ID"))));

    url
}
