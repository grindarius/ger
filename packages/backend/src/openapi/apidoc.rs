use crate::{
    constants::requests::OrderModifier, openapi::security_addon::SecurityAddon,
    routes::forum::posts::get_post_list::GetPostListRequestQueriesOrderByModifier,
};

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        crate::routes::hello::handler,
        crate::routes::auth::signin::handler,
        crate::routes::auth::refresh::handler,
        crate::routes::admin::signup::handler,
        crate::routes::students::signup::handler,
        crate::routes::forum::posts::get_trending_posts_list::handler,
        crate::routes::forum::posts::get_post::handler,
        crate::routes::users::get_user_profile_image::handler,
        crate::routes::users::get_users_list::handler,
        crate::routes::forum::posts::get_trending_posts_list::handler,
        crate::routes::forum::posts::get_post_list::handler,
        crate::routes::forum::posts::get_post_replies::handler,
        crate::routes::forum::categories::get_categories_list::handler,
        crate::routes::forum::categories::get_category::handler
    ),
    components(
        schemas(
            crate::errors::FormattedErrorResponse,
            crate::constants::responses::GetServerInformationResponse,
            crate::constants::responses::DefaultSuccessResponse,
            crate::constants::requests::Order,
            crate::routes::auth::signin::SigninRequestBody,
            crate::routes::students::signup::StudentSignupRequestBody,
            crate::routes::students::signup::StudentSignupRequestBodyInner,
            crate::routes::forum::posts::get_trending_posts_list::GetTrendingPostsListRequestQueries,
            crate::routes::users::get_users_list::GetUsersListRequestQueries,
            crate::routes::admin::signup::AdminSignupRequestBody,
            crate::routes::forum::posts::get_post::GetPostRequestParams,
            crate::routes::forum::posts::get_post::GetPostResponseBody,
            crate::routes::forum::posts::get_post_list::GetPostListRequestQueries,
            crate::routes::forum::posts::get_post_list::GetPostListResponseBody,
            crate::routes::forum::posts::get_post_list::GetPostListResponseBodyInner,
            crate::routes::forum::posts::get_trending_posts_list::GetTrendingPostsListRequestQueries,
            crate::routes::forum::posts::get_trending_posts_list::GetTrendingPostsListResponseBody,
            crate::routes::forum::posts::get_trending_posts_list::GetTrendingPostsListResponseBodyInner,
            crate::routes::forum::posts::get_post_list::GetPostListRequestQueriesOrderBy,
            crate::routes::forum::posts::get_post_list::GetPostListRequestQueries,
            crate::routes::forum::posts::get_post_list::GetPostListResponseBody,
            crate::routes::forum::posts::get_post_list::GetPostListResponseBodyInner,
            crate::routes::forum::posts::get_post_replies::GetPostRepliesRequestParams,
            crate::routes::forum::posts::get_post_replies::GetPostRepliesRequestQueries,
            crate::routes::forum::posts::get_post_replies::GetPostRepliesResponseBody,
            crate::routes::forum::posts::get_post_replies::GetPostRepliesResponseBodyInner,
            crate::routes::forum::categories::get_categories_list::GetCategoriesListRequestQueries,
            crate::routes::forum::categories::get_categories_list::GetCategoriesListResponseBody,
            crate::routes::forum::categories::get_categories_list::GetCategoriesListResponseBodyInner,
            crate::routes::forum::categories::get_category::GetCategoryRequestParams,
            crate::routes::forum::categories::get_category::GetCategoryResponseBody
        )
    ),
    modifiers(
        &SecurityAddon,
        &GetPostListRequestQueriesOrderByModifier,
        &OrderModifier
    ),
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
