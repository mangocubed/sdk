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
    server_url: String,
    title: String,
    pub token: String,
    pub old_tokens: Vec<String>,
}

impl AppConfig {
    pub fn server_url(&self) -> Url {
        self.server_url.parse().expect("Could not parse App server URL")
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_url: "".to_owned(),
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
    provider_url: String,
}

#[cfg(feature = "auth-client")]
impl Default for AuthClientConfig {
    fn default() -> Self {
        Self {
            id: "".to_owned(),
            secret: "".to_owned(),
            provider_url: "".to_owned(),
        }
    }
}

#[cfg(feature = "auth-client")]
impl AuthClientConfig {
    pub fn id(&self) -> uuid::Uuid {
        self.id.parse().expect("Could not parse Auth client ID")
    }

    pub fn provider_url(&self) -> Url {
        self.provider_url
            .parse()
            .expect("Could not parse Auth client provider URL")
    }
}
