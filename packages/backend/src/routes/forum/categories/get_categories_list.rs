use actix_web::{web, HttpResponse};
use ger_from_row::FromRow;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{
    constants::DEFAULT_PAGE,
    constants::{requests::SqlRange, DEFAULT_PAGE_SIZE},
    errors::HttpError,
    shared_app_data::SharedAppData,
};

#[derive(Deserialize, ToSchema, IntoParams, TS)]
#[into_params(parameter_in = Query)]
#[ts(export)]
pub struct GetCategoriesListRequestQueries {
    #[param(minimum = 1, default = json!(DEFAULT_PAGE))]
    #[serde(deserialize_with = "crate::constants::requests::deserialize_page")]
    #[ts(optional)]
    pub page: Option<i32>,
    #[param(minimum = 1, maximum = 100, default = json!(DEFAULT_PAGE_SIZE))]
    #[serde(deserialize_with = "crate::constants::requests::deserialize_page_size")]
    #[ts(optional)]
    pub page_size: Option<i32>,
}

#[derive(Serialize, ToSchema, TS)]
#[ts(export)]
pub struct GetCategoriesListResponseBody {
    categories: Vec<GetCategoriesListResponseBodyInner>,
}

#[derive(Serialize, ToSchema, TS, FromRow)]
#[ts(export)]
pub struct GetCategoriesListResponseBodyInner {
    id: String,
    representative_id: String,
    name: String,
    description: String,
    #[ts(type = "number")]
    post_count: i64,
    latest_post_id: String,
    latest_post_name: String,
    latest_post_user_id: String,
    latest_post_username: String,
    #[serde(with = "time::serde::rfc3339")]
    #[ts(type = "string")]
    latest_post_created_timestamp: time::OffsetDateTime,
}

#[utoipa::path(
    get,
    path = "/forum/categories",
    tag = "forum",
    operation_id = "get_categories_list",
    params(GetCategoriesListRequestQueries),
    responses(
        (
            status = 200,
            description = "successfully get list of forum categories",
            body = GetCategoriesListResponseBody,
            example = json!({ "categories": [] })
        ),
    )
)]
pub async fn handler(
    query: web::Query<GetCategoriesListRequestQueries>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    // Safe unwrap for both variables thanks to the custom deserializer
    let page = query.page.unwrap();
    let page_size = query.page_size.unwrap();

    let SqlRange { limit, offset } = SqlRange::from_page(page, page_size)?;

    let client = data.pool.get().await?;

    let statement = client
        .prepare_typed_cached(
            r##"
            with first_row as (
	            select
                    rank() over (
                        partition by
                            forum_posts.forum_category_id
                        order by
                            forum_posts.forum_post_created_timestamp desc
                    ) as created_rank,
                    forum_posts.forum_category_id as id,
                    forum_categories.forum_category_representative_id as representative_id,
                    forum_categories.forum_category_name as name,
                    forum_categories.forum_category_description as description,
                    forum_posts.forum_post_id as latest_post_id,
                    forum_posts.forum_post_name as latest_post_name,
                    forum_posts.forum_post_created_timestamp as latest_post_created_timestamp,
                    forum_posts.user_id as latest_post_user_id,
                    users.user_username as latest_post_username
                from forum_posts
                inner join forum_categories on forum_posts.forum_category_id = forum_categories.forum_category_id
                inner join users on forum_posts.user_id = users.user_id
            )
            select
                created_rank,
                id,
                representative_id,
                name,
                description,
                latest_post_id,
                latest_post_name,
                latest_post_created_timestamp,
                latest_post_user_id,
                latest_post_username,
                count(forum_posts.forum_post_id) as post_count
            from first_row
            inner join forum_posts on id = forum_posts.forum_category_id
            where created_rank = 1
            group by
                first_row.created_rank,
                first_row.id,
                first_row.representative_id,
                first_row.name,
                first_row.description,
                first_row.latest_post_id,
                first_row.latest_post_name,
                first_row.latest_post_created_timestamp,
                first_row.latest_post_user_id,
                first_row.latest_post_username
            limit $1
            offset $2
            "##,
            &[Type::INT4, Type::INT4],
        )
        .await?;

    let categories = client.query(&statement, &[&limit, &offset]).await?;
    let categories = categories
        .iter()
        .map(|c| GetCategoriesListResponseBodyInner::try_from(c))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(HttpResponse::Ok().json(GetCategoriesListResponseBody { categories }))
}
