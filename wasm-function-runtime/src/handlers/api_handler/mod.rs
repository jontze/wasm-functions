mod deploy_handler;
mod function_handler;
mod scope_handler;
mod variable_handler;

use crate::{domain, server_state::RuntimeStateRef, services::function_service};

pub(crate) use deploy_handler::{CreateHttpFunctionPayload, CreateScheduledFunctionPayload};

pub(crate) fn router(
    app_state: crate::server_state::RuntimeStateRef,
) -> axum::routing::Router<RuntimeStateRef> {
    axum::Router::new()
        .nest("/deploy", deploy_handler::router())
        .nest("/scope", scope_handler::router())
        .nest("/scope/{scope}/variable", variable_handler::router())
        .nest("/scope/{scope}/function", function_handler::router())
        .route_layer(axum::middleware::from_fn_with_state(
            app_state,
            crate::middlewares::auth::auth,
        ))
}
