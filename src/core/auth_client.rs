use std::borrow::Cow;
use std::sync::OnceLock;

use base64::Engine;
use chrono::{DateTime, NaiveDate, Utc};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use url::Url;
use uuid::Uuid;

use super::config::AUTH_CLIENT_CONFIG;

static AUTH_CLIENT: OnceLock<AuthClient> = OnceLock::new();

#[derive(Clone)]
pub struct AuthClient<'a> {
    id: Uuid,
    secret: Cow<'a, str>,
    webhook_secret: Option<Cow<'a, str>>,
    provider_api_url: Url,
}

impl<'a> Default for AuthClient<'a> {
    fn default() -> Self {
        Self {
            id: AUTH_CLIENT_CONFIG.id(),
            secret: Cow::Owned(AUTH_CLIENT_CONFIG.secret.clone()),
            webhook_secret: AUTH_CLIENT_CONFIG.webhook_secret.clone().map(Cow::Owned),
            provider_api_url: AUTH_CLIENT_CONFIG.provider_api_url(),
        }
    }
}

impl<'a> AuthClient<'a> {
    pub fn new() -> &'a Self {
        AUTH_CLIENT.get_or_init(AuthClient::default)
    }

    fn auth_body(&self, auth: &Auth<'_>) -> serde_json::Value {
        serde_json::json!({
            "client_id": self.id,
            "client_secret": self.secret,
            "token": auth.token
        })
    }

    pub async fn refresh_auth(&self, auth: &Auth<'_>) -> anyhow::Result<Auth<'_>> {
        if auth.is_expired() {
            return Err(anyhow::anyhow!("Authorization is expired"));
        }

        let url = self.provider_api_url.join("auth/refresh")?;

        Ok(reqwest::Client::new()
            .put(url)
            .json(&self.auth_body(auth))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn revoke_auth(&self, auth: &Auth<'_>) -> anyhow::Result<()> {
        if auth.is_expired() {
            return Err(anyhow::anyhow!("Authorization is expired"));
        }

        let url = self.provider_api_url.join("auth/revoke")?;

        Ok(reqwest::Client::new()
            .delete(url)
            .json(&self.auth_body(auth))
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn verify_auth(&self, auth: &Auth<'_>) -> anyhow::Result<bool> {
        if auth.is_expired() {
            return Err(anyhow::anyhow!("Authorization is expired"));
        }

        let url = self.provider_api_url.join("auth/verify")?;

        Ok(reqwest::Client::new()
            .get(url)
            .json(&self.auth_body(auth))
            .send()
            .await?
            .status()
            .is_success())
    }

    pub async fn user_info(&self, auth: &Auth<'_>) -> anyhow::Result<UserInfo<'_>> {
        if auth.is_expired() {
            return Err(anyhow::anyhow!("Authorization is expired"));
        }

        let url = self.provider_api_url.join("user-info")?;

        Ok(reqwest::Client::new()
            .get(url)
            .bearer_auth(auth.token.clone())
            .send()
            .await?
            .json()
            .await?)
    }

    pub fn webhook_event(&self, signature: &[u8], body: &[u8]) -> anyhow::Result<WebhookEvent> {
        let Some(webhook_secret) = self.webhook_secret.clone() else {
            return Err(anyhow::anyhow!("Webhook secret is not set"));
        };

        let mut hmac = Hmac::<Sha256>::new_from_slice(webhook_secret.as_bytes())?;
        let signature_decoded = base64::engine::general_purpose::STANDARD.decode(signature)?;

        hmac.update(body);

        hmac.verify_slice(&signature_decoded)?;

        Ok(body.into())
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

    pub async fn refresh(&self) -> anyhow::Result<Self> {
        AuthClient::new().refresh_auth(self).await
    }

    pub async fn revoke(&self) -> anyhow::Result<()> {
        AuthClient::new().revoke_auth(self).await
    }

    pub async fn verify(&self) -> anyhow::Result<bool> {
        AuthClient::new().verify_auth(self).await
    }

    pub async fn user_info(&self) -> anyhow::Result<UserInfo<'a>> {
        AuthClient::new().user_info(self).await
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

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum EventType {
    AuthorizationRevoked,
}

#[derive(Deserialize)]
struct InternalWebhookEvent {
    #[allow(dead_code)]
    event_type: EventType,
    data: serde_json::Value,
}

pub enum WebhookEvent {
    AuthorizationRevoked { token: String },
}

impl From<&[u8]> for WebhookEvent {
    fn from(bytes: &[u8]) -> Self {
        let event: InternalWebhookEvent = serde_json::from_slice(bytes).unwrap();
        let token = event.data.get("token").unwrap().as_str().unwrap().to_string();

        WebhookEvent::AuthorizationRevoked { token }
    }
}
