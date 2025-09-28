use dioxus::core::{DynamicNode, Template, TemplateNode};
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

#[component]
pub fn PageTitle(children: Element) -> Element {
    #[cfg(not(feature = "fullstack"))]
    let mut app_title = "Mango³".to_owned();
    #[cfg(feature = "fullstack")]
    let mut app_title = use_server_cached(|| dioxus::cli_config::app_title().unwrap_or("Mango³".to_owned()));

    if cfg!(debug_assertions) {
        app_title += " (dev)";
    }

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
