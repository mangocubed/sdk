use std::sync::LazyLock;

use figment::Figment;
use figment::providers::{Env, Serialized};
use serde::{Deserialize, Serialize};
use url::Url;

pub fn extract_config_from_env<'a, T>(prefix: &str) -> T
where
    T: Deserialize<'a> + Serialize + Default,
{
    Figment::from(Serialized::defaults(T::default()))
        .merge(Env::prefixed(prefix))
        .extract()
        .unwrap()
}

pub static APP_CONFIG: LazyLock<AppConfig> = LazyLock::new(|| extract_config_from_env("APP_"));

#[cfg(feature = "auth-client")]
pub(crate) static AUTH_CLIENT_CONFIG: LazyLock<AuthClientConfig> =
    LazyLock::new(|| extract_config_from_env("AUTH_CLIENT_"));

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub request_address: String,
    request_url: String,
    title: String,
    pub token: String,
    pub old_tokens: Vec<String>,
}

impl AppConfig {
    pub fn request_url(&self) -> Url {
        self.request_url.parse().expect("Could not parse request URL")
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            request_address: "127.0.0.1:8081".to_owned(),
            request_url: "http://127.0.0.1:8081".to_owned(),
            title: "Mango³".to_owned(),
            token: "".to_owned(),
            old_tokens: Vec::new(),
        }
    }
}

#[cfg(feature = "auth-client")]
#[derive(Deserialize, Serialize)]
pub(crate) struct AuthClientConfig {
    id: String,
    pub secret: String,
    provider_api_url: String,
    provider_app_url: String,
    pub webhook_secret: Option<String>,
}

#[cfg(feature = "auth-client")]
impl Default for AuthClientConfig {
    fn default() -> Self {
        Self {
            id: "".to_owned(),
            secret: "".to_owned(),
            provider_api_url: "http://127.0.0.1:8082".to_owned(),
            provider_app_url: "http://127.0.0.1:8080".to_owned(),
            webhook_secret: None,
        }
    }
}

#[cfg(feature = "auth-client")]
impl AuthClientConfig {
    pub fn id(&self) -> uuid::Uuid {
        self.id.parse().expect("Could not parse Auth client ID")
    }

    pub fn provider_api_url(&self) -> Url {
        self.provider_api_url
            .parse()
            .expect("Could not parse Auth client provider API URL")
    }

    #[allow(dead_code)]
    pub fn provider_app_url(&self) -> Url {
        self.provider_app_url
            .parse()
            .expect("Could not parse Auth client provider App URL")
    }
}
