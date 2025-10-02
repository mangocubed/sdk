use dioxus::prelude::*;

use crate::hooks::{use_field_error_message, use_form};
use crate::icons::{EyeMini, EyeSlashMini};

use super::Modal;

fn on_keydown(event: KeyboardEvent) {
    if event.key() == Key::Enter {
        event.prevent_default();
    }
}

#[component]
pub fn Form(children: Element, #[props(optional)] on_success: Callback) -> Element {
    let form_context = use_form();

    use_effect({
        let form_context = form_context.clone();
        move || {
            if form_context.success().is_some() {
                on_success.call(())
            }
        }
    });

    rsx! {
        form {
            class: "form",
            autocomplete: false,
            novalidate: true,
            onsubmit: move |event| {
                event.prevent_default();

                form_context.callback.call(event.data().values());
            },
            if let Some(form_error) = form_context.error() {
                div { class: "py-2 has-[div:empty]:hidden",
                    div { class: "alert alert-error", role: "alert", {form_error.message.clone()} }
                }
            }

            {children}

            div { class: "py-3 w-full",
                button {
                    class: "btn-submit",
                    onclick: move |event| {
                        if *form_context.is_pending.read() {
                            event.prevent_default();
                        }
                    },
                    r#type: "submit",
                    if *form_context.is_pending.read() {
                        span { class: "loading loading-spinner" }
                    } else {
                        "Submit"
                    }
                }
            }
        }
    }
}

#[component]
fn FormField(children: Element, error: Memo<Option<String>>, label: String) -> Element {
    rsx! {
        fieldset { class: "fieldset",
            legend { class: "fieldset-legend empty:hidden", {label} }

            {children}

            div { class: "label text-error empty:hidden", {error} }
        }
    }
}

#[component]
pub fn FormSuccessModal(#[props(optional)] on_close: Callback) -> Element {
    let form_context = use_form();
    let mut is_open = use_signal(|| false);

    use_effect({
        let form_context = form_context.clone();
        move || {
            is_open.set(form_context.success().is_some());
        }
    });

    rsx! {
        Modal { is_open, is_closable: false,
            if let Some(form_success) = form_context.success() {
                {form_success.message}
            }

            div { class: "modal-action",
                button {
                    class: "btn btn-primary",
                    onclick: {
                        move |event| {
                            event.prevent_default();
                            on_close.call(());
                            is_open.set(false);
                        }
                    },
                    "Ok"
                }
            }
        }
    }
}

#[component]
pub fn PasswordField(id: String, label: String, #[props(default = 256)] max_length: u16, name: String) -> Element {
    let error = use_field_error_message(id.clone());
    let mut input_type = use_signal(|| "password");

    rsx! {
        FormField { error, label,
            div {
                class: "input flex items-center gap-2 pr-0",
                class: if error().is_some() { "input-error" },
                input {
                    class: "grow",
                    id,
                    maxlength: max_length,
                    name,
                    onkeydown: on_keydown,
                    r#type: input_type,
                }

                button {
                    class: "btn btn-ghost btn-sm",
                    onclick: move |event| {
                        event.prevent_default();

                        *input_type.write() = if input_type() == "password" {
                            "text"
                        } else {
                            "password"
                        };
                    },
                    if input_type() == "password" {
                        EyeSlashMini {}
                    } else {
                        EyeMini {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn SelectField(id: String, label: String, name: String, children: Element) -> Element {
    let error = use_field_error_message(id.clone());

    rsx! {
        FormField { error, label,
            select {
                class: "select",
                class: if error().is_some() { "select-error" },
                id,
                name,
                {children}
            }
        }
    }
}

#[component]
pub fn TextField(
    id: String,
    #[props(default = "text".to_owned())] input_type: String,
    label: String,
    #[props(default = 256)] max_length: u16,
    name: String,
    #[props(into, optional)] value: Signal<String>,
) -> Element {
    let error = use_field_error_message(id.clone());

    rsx! {
        FormField { error, label,
            input {
                class: "input",
                class: if error().is_some() { "input-error" },
                id,
                maxlength: max_length,
                name,
                onkeydown: on_keydown,
                oninput: move |event| {
                    *value.write() = event.value();
                },
                r#type: input_type,
                value,
            }
        }
    }
}
