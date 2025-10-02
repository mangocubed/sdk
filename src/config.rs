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
