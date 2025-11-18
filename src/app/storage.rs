use dioxus::CapturedError;
use dioxus::prelude::{Memo, Result, Signal, use_memo};
use dioxus_sdk::storage::{LocalStorage, SessionStorage, get_from_storage, use_storage, use_synced_storage};
use serde::Serialize;
use serde::de::DeserializeOwned;

#[cfg(target_family = "wasm")]
use dioxus::prelude::use_reactive;
#[cfg(target_family = "wasm")]
use web_sys::window;

pub fn use_storage_is_enabled() -> Memo<bool> {
    #[cfg(feature = "server")]
    let is_enabled = false;
    #[cfg(all(not(feature = "server"), target_family = "wasm"))]
    let is_enabled = window().is_some();
    #[cfg(all(not(feature = "server"), not(target_family = "wasm")))]
    let is_enabled = true;

    use_memo(use_reactive!(|is_enabled| { is_enabled }))
}

pub fn get_from_local_storage<T: Clone + DeserializeOwned + PartialEq + Send + Serialize + Sync + 'static>(
    key: &str,
) -> Result<T> {
    get_from_storage::<LocalStorage, Result<T>>(key.to_owned(), || Err(CapturedError::msg("Not found")))
}

pub fn use_local_storage<T: Clone + DeserializeOwned + PartialEq + Send + Serialize + Sync + 'static>(
    key: &str,
) -> Signal<Result<T>> {
    use_synced_storage::<LocalStorage, Result<T>>(key.to_owned(), || Err(CapturedError::msg("Not found")))
}

pub fn use_session_storage<T: Clone + DeserializeOwned + PartialEq + Send + Serialize + Sync + 'static>(
    key: &str,
) -> Signal<Result<T>> {
    use_storage::<SessionStorage, Result<T>>(key.to_owned(), || Err(CapturedError::msg("Not found")))
}
