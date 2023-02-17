use actix_web::{web, HttpResponse};
use ger_from_row::FromRow;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    constants::{SqlRange, DEFAULT_PAGE, DEFAULT_PAGE_SIZE, DEFAULT_TRENDING_WINDOW},
    errors::HttpError,
    shared_app_data::SharedAppData,
};

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetTrendingPostsListRequestQueries {
    /// How big of a window to check for the trending posts. like "trending in the last 24
    /// hours".
    #[param(minimum = 1, default = json!(DEFAULT_TRENDING_WINDOW))]
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    pub hours: Option<i32>,
    /// which page of data to start query.
    #[param(minimum = 1, default = json!(DEFAULT_PAGE))]
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    pub page: Option<i32>,
    /// How much of a post to query in one time.
    #[param(minimum = 1, default = json!(DEFAULT_PAGE_SIZE))]
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    pub page_size: Option<i32>,
}

#[derive(Serialize, ToSchema)]
pub struct GetTrendingPostsListResponseBody {
    posts: Vec<GetTrendingPostsListResponseBodyInner>,
}

#[derive(FromRow, Serialize, ToSchema)]
pub struct GetTrendingPostsListResponseBodyInner {
    id: String,
    name: String,
    username: String,
    #[serde(with = "time::serde::rfc3339")]
    created_timestamp: time::OffsetDateTime,
    view_count: i64,
    vote_count: i64,
}

/// Gets trending posts list with a given page size and time window.
#[utoipa::path(
    get,
    path = "/forum/posts/trending",
    tag = "forum",
    operation_id = "get_trending_posts_list",
    params(GetTrendingPostsListRequestQueries),
    responses(
        (
            status = 200,
            description = "successfully get trending list of forums",
            body = GetTrendingPostsListResponseBody,
            example = json!(GetTrendingPostsListResponseBody { posts: vec![] })
        ),
        (
            status = 400,
            description = "input erorrs",
            body = FormattedErrorResponse,
            example = json!(HttpError::InputValidationError.get_error_struct())
        ),
        (
            status = 401,
            description = "unauthorized",
            body = FormattedErrorResponse,
            example = json!(HttpError::Unauthorized.get_error_struct())
        ),
        (
            status = 500,
            description = "bad errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InternalServerError { cause: "internal".to_string() }.get_error_struct())
        )
    )
)]
pub async fn handler(
    query: web::Query<GetTrendingPostsListRequestQueries>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    let hours = query.hours.unwrap_or(DEFAULT_TRENDING_WINDOW);
    let page = query.page.unwrap_or(DEFAULT_PAGE);
    let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);
    let SqlRange { limit, offset } = SqlRange::from_page(page, page_size)?;

    let client = data.pool.get().await?;

    let statement = client
        .prepare_typed_cached(
            r##"
            select 
                forum_posts.forum_post_id as id,
                forum_posts.forum_post_name as name,
                users.user_username as username,
                forum_posts.forum_post_created_timestamp as created_timestamp,
                count(distinct forum_post_views.user_id) as view_count,
                sum(forum_post_votes.forum_post_vote_increment) as vote_count
            from forum_posts
            inner join users on forum_posts.user_id = users.user_id
            inner join forum_post_views on forum_posts.forum_post_id = forum_post_views.forum_post_id
            inner join forum_post_votes on forum_posts.forum_post_id = forum_post_votes.forum_post_id
            where 
                forum_posts.forum_post_created_timestamp >= now() - interval '$1 hours'
            group by 
                forum_posts.forum_post_id,
                users.user_username
            order by
                vote_count,
                view_count
            limit $2
            offset $3
            "##,
            &[Type::INT4, Type::INT4, Type::INT4],
        )
        .await?;

    let trending_posts = client.query(&statement, &[&hours, &limit, &offset]).await?;
    let trending_posts = trending_posts
        .iter()
        .map(|t| GetTrendingPostsListResponseBodyInner::try_from(t))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HttpResponse::Ok().json(GetTrendingPostsListResponseBody {
        posts: trending_posts,
    }))
}
