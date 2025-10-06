fn main() {
    let app_token = std::env::var("APP_TOKEN").unwrap_or_default();

    println!("cargo:rustc-env=APP_TOKEN={app_token}");
}
