use std::sync::LazyLock;

use figment::Figment;
use figment::providers::{Env, Serialized};
use serde::{Deserialize, Serialize};

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

#[derive(Deserialize, Serialize)]
pub struct AppConfig {
    pub server_url: String,
    pub token: String,
    pub old_tokens: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server_url: "".to_owned(),
            token: "".to_owned(),
            old_tokens: Vec::new(),
        }
    }
}
