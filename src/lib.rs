#[cfg(feature = "dioxus-fullstack")]
use std::collections::HashMap;

#[cfg(feature = "dioxus-fullstack")]
use dioxus::prelude::*;

pub mod constants;

#[cfg(feature = "dioxus")]
pub mod components;
#[cfg(feature = "server")]
pub mod config;
#[cfg(feature = "dioxus")]
mod data_storage;
#[cfg(feature = "dioxus-fullstack")]
pub mod hooks;
#[cfg(feature = "dioxus")]
pub mod icons;
#[cfg(feature = "dioxus-fullstack")]
pub mod serv_fn;

#[cfg(feature = "dioxus-desktop")]
pub use data_storage::set_project_dirs;
#[cfg(feature = "dioxus")]
pub use data_storage::{DataStorage, data_storage};

#[cfg(feature = "dioxus-fullstack")]
static LOADER_UNITS: GlobalSignal<HashMap<String, bool>> = GlobalSignal::new(HashMap::new);

#[cfg(feature = "build")]
pub fn setup_build_env() {
    let app_server_url = std::env::var("APP_SERVER_URL").unwrap_or_default();
    let app_token = std::env::var("APP_TOKEN").unwrap_or_default();

    println!("cargo:rustc-env=APP_SERVER_URL={app_server_url}");
    println!("cargo:rustc-env=APP_TOKEN={app_token}");
}

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

#[cfg(feature = "dioxus-fullstack")]
pub fn loader_is_active() -> bool {
    LOADER_UNITS.read().values().any(|&loading| loading)
}

#[cfg(feature = "dioxus-fullstack")]
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
