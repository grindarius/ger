use crate::openapi::security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::routes::hello::handler,
        crate::routes::auth::signin::handler,
        crate::routes::auth::refresh::handler,
        crate::routes::students::signup::handler
    ),
    components(
        schemas(
            crate::errors::FormattedErrorResponse,
            crate::constants::GetServerInformationResponse,
            crate::routes::auth::signin::SigninBody,
            crate::constants::DefaultSuccessResponse,
            crate::routes::students::signup::StudentSignupBody,
            crate::routes::students::signup::StudentSignupBodyInner
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;
