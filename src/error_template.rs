use http::status::StatusCode;
use leptos::*;
use leptos_router::A;
use thiserror::Error;
use tracing::error;
use crate::css::CssClass::{ErrorPageA, ErrorContainer, ErrorPageH1, ErrorPageP};
use crate::state::GlobalState;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

// A basic function to display errors served by the error boundaries.
// Feel free to do more complicated things here than just displaying the error.
#[component]
pub fn ErrorTemplate(
    #[prop(optional)] outside_errors: Option<Errors>,
    #[prop(optional)] errors: Option<RwSignal<Errors>>,
) -> impl IntoView {
    let errors = match outside_errors {
        Some(e) => create_rw_signal(e),
        None => match errors {
            Some(e) => e,
            None => panic!("No Errors found and we expected errors!"),
        },
    };
    // Get Errors from Signal
    let errors = errors.get_untracked();

    // Downcast lets us take a type that implements `std::error::Error`
    let errors: Vec<AppError> = errors
        .into_iter()
        .filter_map(|(_k, v)| v.downcast_ref::<AppError>().cloned())
        .collect();
    error!("Errors: {errors:#?}");

    // Only the response code for the first error is actually sent from the server
    // this may be customized by the specific application
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let response = use_context::<ResponseOptions>();
        if let Some(response) = response {
            response.set_status(errors[0].status_code());
        }
    }

    let state = use_context::<RwSignal<GlobalState>>().unwrap_or_default();
    view! {
        <div class={move || ErrorContainer.get_css(state.get().theme)}>
            <For
                // a function that returns the items we're iterating over; a signal is fine
                each= move || {errors.clone().into_iter().enumerate()}
                // a unique key for each item as a reference
                key=|(index, _error)| *index
                // renders each item to a view
                children=move |error| {
                    let error_string = error.1.to_string();
                    let error_code= error.1.status_code();
                    view! {
                        <h1 class={move || ErrorPageH1.get_css(state.get().theme)}>{error_code.to_string()}</h1>
                        <p class={move || ErrorPageP.get_css(state.get().theme)} >"Error: " {error_string}</p>
                        <A href="/" class={move || ErrorPageA.get_css(state.get().theme)}>"返回首页"</A>
                    }
                }
            />
        </div>
    }
}
