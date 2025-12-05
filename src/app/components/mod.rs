use dioxus::core::{DynamicNode, Template, TemplateNode};
use dioxus::prelude::*;

use super::icons::Mango3Icon;

mod app_provider;
mod form;
mod logo;
mod modal;

pub use app_provider::*;
pub use form::*;
pub use logo::*;
pub use modal::*;

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
pub fn H2(children: Element) -> Element {
    rsx! {
        h2 { class: "h2", {children} }
    }
}

#[component]
pub fn H3(children: Element) -> Element {
    rsx! {
        h3 { class: "h3", {children} }
    }
}

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        div { class: "navbar", {children} }
    }
}

#[component]
pub fn NavbarCenter(#[props(optional)] class: String, children: Element) -> Element {
    rsx! {
        div { class: format!("navbar-center {class}"), {children} }
    }
}

#[component]
pub fn NavbarEnd(#[props(optional)] class: String, children: Element) -> Element {
    rsx! {
        div { class: format!("navbar-end {class}"), {children} }
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
