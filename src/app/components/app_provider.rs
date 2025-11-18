use dioxus::prelude::*;

use crate::app::icons::Mango3Icon;
use crate::app::spinner_is_active;

#[derive(Clone)]
pub struct AppTitle(pub String);

#[component]
pub fn AppProvider(children: Element, #[props(optional)] is_starting: ReadSignal<bool>) -> Element {
    let mut app_title = env!("APP_TITLE").to_owned();

    if cfg!(debug_assertions) {
        app_title += " (dev)";
    }

    use_context_provider(|| AppTitle(app_title.clone()));

    rsx! {
        {children}

        div { class: "spinner", class: if !spinner_is_active() { "hidden" } }

        div { class: "splash", class: if !is_starting() { "splash-hidden" },
            figure {
                div { class: "splash-pulse" }

                Mango3Icon {}
            }
        }

    }
}
