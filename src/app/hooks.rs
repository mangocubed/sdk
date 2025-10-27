use std::collections::HashMap;

use dioxus::prelude::*;
use serde_json::Value;

use super::run_with_spinner;
use super::{ActionError, ActionResult, ActionSuccess};

#[derive(Clone, PartialEq)]
pub struct FormProvider {
    pub(crate) callback: Callback<Vec<(String, FormValue)>>,
    result: Signal<Option<ActionResult>>,
    pub(crate) is_pending: Signal<bool>,
}

impl FormProvider {
    pub(crate) fn success(&self) -> Option<ActionSuccess> {
        self.result.read().as_ref().and_then(|result| result.clone().ok())
    }

    pub(crate) fn error(&self) -> Option<ActionError> {
        self.result.read().as_ref().and_then(|result| result.clone().err())
    }

    pub(crate) fn field_error_message(&self, id: &str) -> Option<String> {
        self.error()
            .and_then(|error| error.details)
            .and_then(|validation_errors| validation_errors.field_errors().get(id).cloned().cloned())
            .and_then(|field_errors| {
                field_errors
                    .iter()
                    .find_map(|error| error.message.as_ref().map(|message| message.to_string()))
            })
    }

    pub fn reset(&mut self) {
        if *self.is_pending.peek() {
            return;
        }

        self.result.set(None);
    }
}

pub fn use_action_with_spinner<F, I, T>(id: &'static str, future: impl Fn(I) -> F + Clone + 'static) -> Action<(I,), T>
where
    F: Future<Output = Result<T>> + 'static,
    I: Clone + 'static,
    T: 'static,
{
    use_action(move |input: I| {
        let future = future.clone();

        run_with_spinner(id, move || future(input.clone()))
    })
}

pub(crate) fn use_form() -> FormProvider {
    use_context()
}

pub(crate) fn use_field_error_message(id: String) -> Memo<Option<String>> {
    let form_context = use_form();

    use_memo(move || form_context.field_error_message(&id))
}

pub fn use_form_provider<FA: Fn(Value) -> R + Copy + 'static, R: IntoFuture<Output = ActionResult>>(
    id: &'static str,
    future: FA,
) -> FormProvider {
    let mut is_pending = use_signal(|| false);
    let mut result = use_signal(|| None);

    let callback = use_callback(move |input: Vec<(String, FormValue)>| {
        is_pending.set(true);

        spawn(async move {
            *result.write() = Some(
                run_with_spinner(id, move || {
                    let input = serde_json::to_value(
                        input
                            .iter()
                            .filter_map(|(name, value)| {
                                if let FormValue::Text(text) = value {
                                    Some((name.clone(), Value::String(text.clone())))
                                } else {
                                    None
                                }
                            })
                            .collect::<HashMap<String, Value>>(),
                    )
                    .expect("Could not get input");

                    async move { future(input).await }
                })
                .await,
            );

            is_pending.set(false);
        });
    });

    use_context_provider(|| FormProvider {
        callback,
        is_pending,
        result,
    })
}

pub fn use_resource_with_spinner<T, F>(id: &'static str, future: impl Fn() -> F + Clone + 'static) -> Resource<T>
where
    T: 'static,
    F: Future<Output = T> + 'static,
{
    use_resource(move || run_with_spinner(id, future.clone()))
}
