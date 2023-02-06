use crate::openapi::security_addon::SecurityAddon;

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::routes::hello::handler,
        crate::routes::auth::signin::handler,
        crate::routes::auth::refresh::handler,
        crate::routes::students::signup::handler,
        crate::routes::forum::announcements::get_announcements_list::handler,
        crate::routes::forum::get_trending_posts_list::handler
    ),
    components(
        schemas(
            crate::errors::FormattedErrorResponse,
            crate::constants::responses::GetServerInformationResponse,
            crate::routes::auth::signin::SigninRequestBody,
            crate::constants::responses::DefaultSuccessResponse,
            crate::routes::students::signup::StudentSignupRequestBody,
            crate::routes::students::signup::StudentSignupRequestBodyInner,
            crate::routes::forum::announcements::get_announcements_list::GetAnnouncementsListResponseBody,
            crate::routes::forum::announcements::get_announcements_list::GetAnnouncementsListResponseBodyInner,
            crate::routes::forum::get_trending_posts_list::GetTrendingPostsListRequestQueries
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
        )
    )
)]
pub struct ApiDoc;
