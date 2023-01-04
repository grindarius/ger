use crate::openapi::security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::routes::hello::handler,
    ),
    components(
        schemas(
            crate::errors::FormattedErrorResponse
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
