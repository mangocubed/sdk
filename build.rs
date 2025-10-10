fn main() {
    let app_server_url = std::env::var("APP_SERVER_URL").unwrap_or_default();
    let app_token = std::env::var("APP_TOKEN").unwrap_or_default();
    let auth_client_id = std::env::var("AUTH_CLIENT_ID").unwrap_or_default();
    let auth_client_provider_url = std::env::var("AUTH_CLIENT_PROVIDER_URL").unwrap_or_default();

    println!("cargo:rustc-env=APP_SERVER_URL={app_server_url}");
    println!("cargo:rustc-env=APP_TOKEN={app_token}");
    println!("cargo:rustc-env=AUTH_CLIENT_ID={auth_client_id}");
    println!("cargo:rustc-env=AUTH_CLIENT_PROVIDER_URL={auth_client_provider_url}");
}
