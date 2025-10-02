use std::collections::HashMap;

use dioxus::prelude::*;

pub mod components;
pub mod constants;
pub mod hooks;
pub mod icons;

#[cfg(feature = "server")]
pub mod config;
#[cfg(feature = "fullstack")]
pub mod serv_fn;

static LOADER_UNITS: GlobalSignal<HashMap<String, bool>> = GlobalSignal::new(HashMap::new);

pub fn loader_is_active() -> bool {
    LOADER_UNITS.read().values().any(|&loading| loading)
}

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
