#[cfg(feature = "dioxus")]
use std::collections::HashMap;

#[cfg(feature = "dioxus")]
use dioxus::prelude::*;

pub mod constants;

#[cfg(feature = "dioxus")]
pub mod components;
#[cfg(feature = "server")]
pub mod config;
#[cfg(feature = "dioxus")]
pub mod hooks;
#[cfg(feature = "dioxus")]
pub mod icons;
#[cfg(feature = "dioxus-fullstack")]
pub mod serv_fn;

#[cfg(feature = "dioxus")]
static LOADER_UNITS: GlobalSignal<HashMap<String, bool>> = GlobalSignal::new(HashMap::new);

#[cfg(feature = "server")]
pub fn generate_random_string(length: u8) -> String {
    use rand::distr::Alphanumeric;
    use rand::{Rng, rng};

    rng()
        .sample_iter(&Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}

#[cfg(feature = "dioxus")]
pub fn loader_is_active() -> bool {
    LOADER_UNITS.read().values().any(|&loading| loading)
}

#[cfg(feature = "dioxus")]
async fn run_with_loader<T, F>(id: String, mut future: impl FnMut() -> F + 'static) -> T
where
    T: 'static,
    F: IntoFuture<Output = T> + 'static,
{
    LOADER_UNITS.write().insert(id.clone(), true);

    let resp = future().await;

    LOADER_UNITS.write().insert(id.clone(), false);

    resp
}
