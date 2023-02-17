use actix_web::{web, HttpResponse};
use ger_from_row::FromRow;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    constants::{SqlRange, DEFAULT_PAGE, DEFAULT_PAGE_SIZE},
    errors::HttpError,
    shared_app_data::SharedAppData,
};

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GetPostListRequestQueries {
    /// get list of global announcements
    #[param(default = json!(false))]
    pub announcement: Option<bool>,
    /// get list of category based announcements
    #[param(default = json!(false))]
    pub category_based_announcement: Option<bool>,
    /// page of the queried data
    #[param(minimum = 1, default = json!(DEFAULT_PAGE))]
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    pub page: Option<i32>,
    /// size of page for each query
    #[param(minimum = 1, default = json!(DEFAULT_PAGE_SIZE))]
    #[serde(default, deserialize_with = "crate::constants::empty_string_as_none")]
    pub page_size: Option<i32>,
}

#[derive(Serialize, ToSchema)]
pub struct GetPostListResponseBody {
    posts: Vec<GetPostListResponseBodyInner>,
}

#[derive(FromRow, Serialize, ToSchema)]
pub struct GetPostListResponseBodyInner {
    id: String,
    username: String,
    name: String,
    view_count: i64,
    vote_count: i64,
    #[serde(with = "time::serde::rfc3339")]
    created_timestamp: time::OffsetDateTime,
}

#[utoipa::path(
    get,
    path = "/forum/posts",
    tag = "forum",
    operation_id = "get_post_list",
    params(GetPostListRequestQueries),
    responses(
        (
            status = 200,
            description = "successfully get list of posts",
            body = GetPostListResponseBody,
            example = json!({ "posts": [] })
        )
    )
)]
pub async fn handler(
    query: web::Query<GetPostListRequestQueries>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    let announcement = query.announcement.unwrap_or(false);
    let category_based_announcement = query.category_based_announcement.unwrap_or(false);
    let page = query.page.unwrap_or(DEFAULT_PAGE);
    let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

    let SqlRange { limit, offset } = SqlRange::from_page(page, page_size)?;

    let client = data.pool.get().await?;

    let statement = client
        .prepare_typed_cached(
            r##"
            select
                forum_posts.forum_post_id as id,
                users.user_username as username,
                forum_posts.forum_post_name as name,
                count(distinct forum_post_views.user_id) as view_count,
                sum(forum_post_votes.forum_post_vote_increment) as vote_count,
                forum_posts.forum_post_created_timestamp as created_timestamp
            from forum_posts
            inner join users on forum_posts.user_id = users.user_id
            inner join forum_post_views on forum_posts.forum_post_id = forum_post_views.forum_post_id
            inner join forum_post_votes on forum_posts.forum_post_id = forum_post_votes.forum_post_id
            where
                forum_posts.forum_post_is_global_announcement = $1 and
                forum_posts.forum_post_is_category_based_announcement = $2
            group by
                forum_posts.forum_post_id,
                users.user_username
            order by forum_posts.forum_post_created_timestamp desc
            limit $3
            offset $4
            "##,
            &[Type::BOOL, Type::BOOL, Type::INT4, Type::INT4],
        )
        .await?;

    let posts = client
        .query(
            &statement,
            &[&announcement, &category_based_announcement, &limit, &offset],
        )
        .await?;

    let posts = match posts
        .iter()
        .map(|p| GetPostListResponseBodyInner::try_from(p))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(p) => p,
        Err(e) => {
            tracing::error!("{}", e.to_string());
            return Err(HttpError::InternalServerError {
                cause: e.to_string(),
            });
        }
    };

    return Ok(HttpResponse::Ok().json(GetPostListResponseBody { posts }));
}
