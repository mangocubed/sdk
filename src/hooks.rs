use std::collections::HashMap;

use dioxus::prelude::*;
use serde::de::DeserializeOwned;

use crate::run_with_loader;
use crate::serv_fn::{FormError, FormResult, FormSuccess};

#[derive(Clone, PartialEq)]
pub struct FormProvider {
    pub(crate) callback: Callback<HashMap<String, FormValue>>,
    result: Signal<Option<FormResult>>,
    pub(crate) is_pending: Signal<bool>,
}

impl FormProvider {
    pub(crate) fn success(&self) -> Option<FormSuccess> {
        self.result.read().as_ref().and_then(|result| result.clone().ok())
    }

    pub(crate) fn error(&self) -> Option<FormError> {
        self.result
            .read()
            .as_ref()
            .and_then(|result| {
                if let Err(ServerFnError::ServerError(form_error)) = result {
                    Some(form_error)
                } else {
                    None
                }
            })
            .cloned()
    }

    pub(crate) fn field_error_message(&self, id: &str) -> Option<String> {
        self.error()
            .and_then(|error| error.validation_errors)
            .and_then(|validation_errors| validation_errors.field_errors().get(id).cloned().cloned())
            .and_then(|field_errors| {
                field_errors
                    .iter()
                    .find_map(|error| error.message.as_ref().map(|message| message.to_string()))
            })
    }

    pub fn reset(&mut self) {
        if *self.is_pending.read() {
            return;
        }

        self.result.set(None);
    }
}

pub(crate) fn use_form() -> FormProvider {
    use_context()
}

pub(crate) fn use_field_error_message(id: String) -> Memo<Option<String>> {
    let form_context = use_form();

    use_memo(move || form_context.field_error_message(&id))
}

pub fn use_form_provider<
    FA: Fn(I) -> R + Copy + 'static,
    I: Clone + DeserializeOwned + 'static,
    R: IntoFuture<Output = FormResult>,
>(
    id: &'static str,
    future: FA,
) -> FormProvider {
    let mut is_pending = use_signal(|| false);
    let mut result = use_signal(|| None);

    let callback = use_callback(move |input: HashMap<String, FormValue>| {
        is_pending.set(true);

        spawn(async move {
            *result.write() = Some(
                run_with_loader(id, move || {
                    let input = serde_json::from_value(
                        input
                            .iter()
                            .map(|(name, value)| (name.clone(), value.as_value()))
                            .collect(),
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

pub fn use_resource_with_loader<T, F>(id: &'static str, future: impl FnMut() -> F + Clone + 'static) -> Resource<T>
where
    T: 'static,
    F: Future<Output = T> + 'static,
{
    use_resource(move || run_with_loader(id, future.clone()))
}
