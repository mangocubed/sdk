use dioxus::prelude::*;

use crate::app::sleep;

#[component]
pub fn Modal(
    children: Element,
    #[props(optional)] class: String,
    is_open: Signal<bool>,
    #[props(default = true)] is_closable: bool,
    #[props(optional)] on_open: Callback,
    #[props(optional)] on_close: Callback,
) -> Element {
    let mut is_visible = use_signal(move || *is_open.read());

    use_effect(move || {
        if !*is_open.read() && *is_visible.read() {
            on_close.call(());

            spawn(async move {
                sleep(300).await;
                is_visible.set(false);
            });
        }
    });

    rsx! {
        if is_open() || is_visible() {
            dialog {
                class: "modal",
                class: if is_open() && is_visible() { "modal-open" },
                onmounted: move |_| async move {
                    sleep(5).await;
                    is_visible.set(true);
                    sleep(300).await;
                    on_open.call(());
                },
                if is_closable {
                    button {
                        class: "modal-close",
                        onclick: move |_| is_open.set(false),
                        "✕"
                    }
                }

                div { class: format!("modal-box {class}"), {children} }

                if is_closable {
                    div {
                        class: "modal-backdrop",
                        onclick: move |_| is_open.set(false),
                    }
                }
            }
        }
    }
}
