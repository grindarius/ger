use crate::openapi::security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::routes::hello::handler,
        crate::routes::auth::signin::handler,
        crate::routes::auth::refresh::handler,
        crate::routes::admin::signup::handler,
        crate::routes::students::signup::handler,
        crate::routes::forum::posts::get_trending_posts_list::handler,
        crate::routes::users::get_user_profile_image::handler,
        crate::routes::users::get_users_list::handler,
        crate::routes::forum::posts::get_post::handler
    ),
    components(
        schemas(
            crate::errors::FormattedErrorResponse,
            crate::constants::responses::GetServerInformationResponse,
            crate::routes::auth::signin::SigninRequestBody,
            crate::constants::responses::DefaultSuccessResponse,
            crate::routes::students::signup::StudentSignupRequestBody,
            crate::routes::students::signup::StudentSignupRequestBodyInner,
            crate::routes::forum::posts::get_trending_posts_list::GetTrendingPostsListRequestQueries,
            crate::routes::users::get_users_list::GetUsersListQueries,
            crate::routes::admin::signup::AdminSignupRequestBody,
            crate::routes::forum::posts::get_post::GetPostRequestParams,
            crate::routes::forum::posts::get_post::GetPostResponseBody
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (
            name = "home"
        ),
        (
            name = "auth"
        ),
        (
            name = "forum"
        ),
        (
            name = "students"
        ),
        (
            name = "users"
        )
    )
)]
pub struct ApiDoc;
