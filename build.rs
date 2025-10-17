fn main() {
    let app_request_url = std::env::var("APP_REQUEST_URL").unwrap_or("http://127.0.0.1:8081".to_owned());
    let app_title = std::env::var("APP_TITLE").unwrap_or("Mango³".to_owned());
    let app_token = std::env::var("APP_TOKEN").unwrap_or_default();
    let auth_client_id = std::env::var("AUTH_CLIENT_ID").unwrap_or_default();
    let auth_client_provider_app_url = std::env::var("AUTH_CLIENT_PROVIDER_APP_URL").unwrap_or_default();

    println!("cargo:rustc-env=APP_REQUEST_URL={app_request_url}");
    println!("cargo:rustc-env=APP_TITLE={app_title}");
    println!("cargo:rustc-env=APP_TOKEN={app_token}");
    println!("cargo:rustc-env=AUTH_CLIENT_ID={auth_client_id}");
    println!("cargo:rustc-env=AUTH_CLIENT_PROVIDER_APP_URL={auth_client_provider_app_url}");
}
