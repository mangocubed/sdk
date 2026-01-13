use dioxus::prelude::*;
use serde_json::Value;

use crate::app::hooks::{use_field_error_message, use_form};
use crate::app::icons::{EyeMini, EyeSlashMini};

use super::Modal;

fn on_keydown(event: KeyboardEvent) {
    if event.key() == Key::Enter {
        event.prevent_default();
    }
}

#[component]
pub fn Form(children: Element, #[props(optional)] on_success: Callback<Value>) -> Element {
    let form_context = use_form();

    use_effect({
        let form_context = form_context.clone();
        move || {
            if let Some(form_success) = form_context.success() {
                on_success.call(form_success.data)
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
fn FormField(
    children: Element,
    #[props(into, optional)] disabled: Signal<bool>,
    error: Memo<Option<String>>,
    #[props(optional)] label: String,
) -> Element {
    rsx! {
        fieldset { class: "fieldset", disabled,
            legend { class: "fieldset-legend empty:hidden", {label} }

            {children}

            div { class: "label text-error empty:hidden", {error} }
        }
    }
}

#[component]
pub fn FormSuccessModal(#[props(optional)] on_close: Callback<Value>) -> Element {
    let form_context = use_form();
    let mut is_open = use_signal(|| false);
    let form_success = use_memo(move || form_context.success());

    use_effect(move || {
        is_open.set(form_success().is_some());
    });

    rsx! {
        Modal { is_open, is_closable: false,
            if let Some(form_success) = form_success() {
                {form_success.message}
            }

            div { class: "modal-action",
                button {
                    class: "btn btn-primary",
                    onclick: {
                        move |event| {
                            event.prevent_default();
                            if let Some(form_success) = form_success() {
                                on_close.call(form_success.data);
                            }
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
pub fn PasswordField(
    #[props(into, optional)] disabled: Signal<bool>,
    id: String,
    label: String,
    #[props(default = 256)] max_length: u16,
    name: String,
    #[props(into, optional)] readonly: Signal<bool>,
) -> Element {
    let error = use_field_error_message(id.clone());
    let mut input_type = use_signal(|| "password");

    rsx! {
        FormField { disabled, error, label,
            div {
                class: "input flex items-center gap-2 pr-0",
                class: if error().is_some() { "input-error" },
                input {
                    class: "grow",
                    disabled,
                    id,
                    maxlength: max_length,
                    name,
                    onkeydown: on_keydown,
                    readonly,
                    r#type: input_type,
                }

                button {
                    class: "btn btn-ghost btn-sm",
                    disabled,
                    onclick: move |event| {
                        event.prevent_default();

                        if *readonly.read() {
                            return;
                        }

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
pub fn SelectField(
    children: Element,
    #[props(into, optional)] disabled: Signal<bool>,
    id: String,
    label: String,
    name: String,
) -> Element {
    let error = use_field_error_message(id.clone());

    rsx! {
        FormField { disabled, error, label,
            select {
                class: "select",
                class: if error().is_some() { "select-error" },
                disabled,
                id,
                name,
                {children}
            }
        }
    }
}

#[component]
pub fn TextField(
    #[props(into, optional)] disabled: Signal<bool>,
    id: String,
    #[props(default = "text".to_owned())] input_type: String,
    label: String,
    #[props(default = 256)] max_length: u16,
    name: String,
    #[props(into, optional)] readonly: Signal<bool>,
    #[props(into, optional)] value: Signal<String>,
) -> Element {
    let error = use_field_error_message(id.clone());

    rsx! {
        FormField { disabled, error, label,
            input {
                class: "input",
                class: if error().is_some() { "input-error" },
                disabled,
                id,
                maxlength: max_length,
                name,
                onkeydown: on_keydown,
                oninput: move |event| {
                    *value.write() = event.value();
                },
                readonly,
                r#type: input_type,
                value,
            }
        }
    }
}

#[component]
pub fn ToggleField(
    #[props(into, optional)] disabled: Signal<bool>,
    id: String,
    label: String,
    name: String,
    #[props(into, optional)] readonly: Signal<bool>,
    #[props(into, optional)] checked: ReadSignal<bool>,
    #[props(default = "false".to_owned())] unchecked_value: String,
    #[props(default = "true".to_owned())] checked_value: String,
) -> Element {
    let error = use_field_error_message(id.clone());

    rsx! {
        FormField { disabled, error,
            if !checked() {
                input {
                    disabled,
                    r#type: "hidden",
                    name: name.clone(),
                    readonly,
                    value: unchecked_value,
                }
            }

            label { class: "fieldset-legend",
                span { {label} }

                input {
                    checked,
                    class: "toggle",
                    class: if error().is_some() { "toggle-error" },
                    disabled,
                    id,
                    name,
                    readonly,
                    r#type: "checkbox",
                    value: checked_value,
                }
            }
        }
    }
}
