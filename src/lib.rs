#[cfg(feature = "dioxus-fullstack")]
use std::collections::HashMap;

#[cfg(any(feature = "dioxus-fullstack", feature = "dioxus-web"))]
use dioxus::prelude::*;

#[cfg(any(feature = "app", feature = "dioxus"))]
pub mod app;

pub mod constants;

#[cfg(feature = "auth-client")]
pub mod auth_client;
#[cfg(feature = "server")]
pub mod config;
#[cfg(feature = "dioxus")]
pub mod data_storage;
#[cfg(feature = "dioxus-fullstack")]
pub mod hooks;
#[cfg(any(feature = "dioxus", feature = "app"))]
pub mod icons;
#[cfg(feature = "test-utils")]
pub mod test_utils;

#[cfg(feature = "dioxus-fullstack")]
static LOADER_UNITS: GlobalSignal<HashMap<String, bool>> = GlobalSignal::new(HashMap::new);

#[cfg(feature = "build")]
pub fn setup_build_env() {
    let app_server_url = std::env::var("APP_SERVER_URL").unwrap_or_default();
    let app_token = std::env::var("APP_TOKEN").unwrap_or_default();

    println!("cargo:rustc-env=APP_SERVER_URL={app_server_url}");
    println!("cargo:rustc-env=APP_TOKEN={app_token}");

    #[cfg(feature = "auth-client")]
    {
        let auth_client_id = std::env::var("AUTH_CLIENT_ID").unwrap_or_default();
        let auth_client_provider_url = std::env::var("AUTH_CLIENT_PROVIDER_URL").unwrap_or_default();

        println!("cargo:rustc-env=AUTH_CLIENT_ID={auth_client_id}");
        println!("cargo:rustc-env=AUTH_CLIENT_PROVIDER_URL={auth_client_provider_url}");
    }
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

#[cfg(feature = "dioxus-web")]
pub fn open_external_url(value: url::Url) {
    navigator().push(value.to_string());
}

#[cfg(feature = "dioxus-desktop")]
pub fn open_external_url(value: url::Url) {
    let _ = dioxus::desktop::use_window().webview.load_url(value.as_ref());
}

#[cfg(feature = "dioxus-mobile")]
pub fn open_external_url(value: url::Url) {
    let _ = dioxus::mobile::use_window().webview.load_url(value.as_ref());
}

#[cfg(feature = "dioxus-server")]
pub fn open_external_url(_value: url::Url) {}

#[cfg(feature = "dioxus-fullstack")]
pub async fn run_with_loader<T, F>(id: &str, mut future: impl FnMut() -> F + 'static) -> T
where
    T: 'static,
    F: IntoFuture<Output = T> + 'static,
{
    LOADER_UNITS.write().insert(id.to_owned(), true);

    let resp = future().await;

    LOADER_UNITS.write().insert(id.to_owned(), false);

    resp
}
