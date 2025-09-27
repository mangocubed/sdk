use dioxus::prelude::*;

#[component]
pub fn ConfirmationModal(children: Element, is_open: Signal<bool>, on_accept: Callback) -> Element {
    rsx! {
        Modal { is_closable: false, is_open,
            div { {children} }

            div { class: "modal-action",
                button {
                    class: "btn",
                    onclick: move |event| {
                        event.prevent_default();
                        *is_open.write() = false;
                    },
                    "Cancel"
                }
                button {
                    class: "btn btn-primary",
                    onclick: move |event| {
                        event.prevent_default();
                        *is_open.write() = false;
                        on_accept.call(());
                    },
                    "Accept"
                }
            }
        }
    }
}

#[component]
pub fn Footer(children: Element) -> Element {
    rsx! {
        footer { class: "footer md:footer-horizontal bg-base-200 p-10", {children} }
    }
}

#[component]
pub fn Modal(
    children: Element,
    #[props(optional)] class: String,
    is_open: Signal<bool>,
    #[props(default = true)] is_closable: bool,
    #[props(optional)] on_close: Callback<MouseEvent>,
) -> Element {
    let on_close = move |event: MouseEvent| {
        event.prevent_default();
        *is_open.write() = false;
        on_close.call(event);
    };

    rsx! {
        dialog { class: "modal", class: if is_open() { "modal-open" },
            if is_closable {
                button {
                    class: "btn btn-sm btn-circle btn-ghost absolute right-2 top-2",
                    onclick: on_close,
                    "✕"
                }
            }

            div { class: format!("modal-box {class}"), {children} }

            if is_closable {
                div { class: "modal-backdrop", onclick: on_close }
            }
        }
    }
}

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        div { class: "navbar bg-base-300 shadow-md px-3", {children} }
    }
}

#[component]
pub fn NavbarEnd(children: Element) -> Element {
    rsx! {
        div { class: "navbar-end", {children} }
    }
}

#[component]
pub fn NavbarStart(children: Element) -> Element {
    rsx! {
        div { class: "navbar-start", {children} }
    }
}
