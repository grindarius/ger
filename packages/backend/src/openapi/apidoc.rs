use crate::openapi::security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::routes::hello::handler,
        crate::routes::auth::signin::handler,
    ),
    components(
        schemas(
            crate::errors::FormattedErrorResponse,
            crate::constants::GetServerInformationResponse,
            crate::routes::auth::signin::SigninBody,
            crate::constants::DefaultSuccessResponse,
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
