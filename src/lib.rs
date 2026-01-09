#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "core")]
pub mod core;
#[cfg(feature = "monitor")]
pub mod monitor;

pub mod constants;

#[cfg(feature = "test-utils")]
pub mod test_utils;

#[cfg(feature = "build")]
pub fn setup_build_env() {
    println!("cargo:rerun-if-env-changed=APP_SERVER_URL");
    println!("cargo:rerun-if-env-changed=APP_TITLE");
    println!("cargo:rerun-if-env-changed=APP_TOKEN");
    println!("cargo:rerun-if-env-changed=AUTH_CLIENT_ID");
    println!("cargo:rerun-if-env-changed=AUTH_CLIENT_PROVIDER_APP_URL");

    let app_server_url = std::env::var("APP_SERVER_URL").unwrap_or_default();
    let app_title = std::env::var("APP_TITLE").unwrap_or("Mango³".to_owned());
    let app_token = std::env::var("APP_TOKEN").unwrap_or_default();
    let auth_client_id = std::env::var("AUTH_CLIENT_ID").unwrap_or_default();
    let auth_client_provider_app_url = std::env::var("AUTH_CLIENT_PROVIDER_APP_URL").unwrap_or_default();

    println!("cargo:rustc-env=APP_SERVER_URL={app_server_url}");
    println!("cargo:rustc-env=APP_TITLE={app_title}");
    println!("cargo:rustc-env=APP_TOKEN={app_token}");
    println!("cargo:rustc-env=AUTH_CLIENT_ID={auth_client_id}");
    println!("cargo:rustc-env=AUTH_CLIENT_PROVIDER_APP_URL={auth_client_provider_app_url}");
}

pub trait AsyncInto<T> {
    fn async_into(&self) -> impl std::future::Future<Output = T>;
}
