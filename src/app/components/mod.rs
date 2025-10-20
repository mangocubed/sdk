use dioxus::core::{DynamicNode, Template, TemplateNode};
use dioxus::prelude::*;

use super::icons::Mango3Icon;

mod logo;

mod form;

pub use logo::*;

pub use form::*;

#[derive(Clone)]
struct AppTitle(String);

#[component]
pub fn AppProvider(children: Element, #[props(optional)] is_starting: ReadSignal<bool>) -> Element {
    let mut app_title = env!("APP_TITLE").to_owned();

    if cfg!(debug_assertions) {
        app_title += " (dev)";
    }

    use_context_provider(|| AppTitle(app_title.clone()));

    rsx! {
        {children}

        Spinner {}

        div { class: "splash", class: if !is_starting() { "splash-hidden" },
            figure {
                div { class: "splash-pulse" }

                Mango3Icon {}
            }
        }

    }
}

#[component]
pub fn Brand(children: Element) -> Element {
    let AppTitle(app_title) = use_context();

    rsx! {
        div { class: "brand", title: app_title,
            Mango3Icon { class: "brand-icon" }

            Mango3Logo { class: "brand-logo" }

            div { class: "brand-suffix", {children} }

            if cfg!(debug_assertions) {
                div { class: "brand-dev", "(dev)" }
            }
        }
    }
}

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
pub fn Dropdown(#[props(optional)] class: String, children: Element) -> Element {
    rsx! {
        div { class: format!("dropdown cursor-pointer {class}"), {children} }
    }
}

#[component]
pub fn DropdownContent(#[props(optional)] class: String, children: Element) -> Element {
    rsx! {
        div { class: format!("dropdown-content {class}"), {children} }
    }
}

#[component]
pub fn Footer(children: Element) -> Element {
    rsx! {
        footer { class: "footer", {children} }
    }
}

#[component]
pub fn H1(children: Element) -> Element {
    rsx! {
        h1 { class: "h1", {children} }
    }
}

#[component]
fn Spinner() -> Element {
    use super::spinner_is_active;

    rsx! {
        div { class: "spinner", class: if !spinner_is_active() { "hidden" } }
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
                button { class: "modal-close", onclick: on_close, "✕" }
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
        div { class: "navbar", {children} }
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

#[component]
pub fn PageTitle(children: Element) -> Element {
    let AppTitle(app_title) = use_context();

    let vnode = children?;
    let page_title = match vnode.template {
        Template {
            roots: &[TemplateNode::Text { text }],
            node_paths: &[],
            attr_paths: &[],
            ..
        } => text.to_string(),
        Template {
            roots: &[TemplateNode::Dynamic { id }],
            node_paths: &[&[0]],
            attr_paths: &[],
            ..
        } => {
            let node = &vnode.dynamic_nodes[id];
            match node {
                DynamicNode::Text(text) => text.value.clone(),
                _ => {
                    return rsx!();
                }
            }
        }
        _ => {
            return rsx!();
        }
    };

    rsx! {
        document::Title { "{page_title} | {app_title}" }
    }
}
