use crate::openapi::security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::routes::hello::handler,
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
