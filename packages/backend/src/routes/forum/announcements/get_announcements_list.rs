use actix_web::{web, HttpResponse};
use ger_from_row::FromRow;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

use crate::{
    constants::{swagger::AuthenticationHeaders, DEFAULT_PAGE, DEFAULT_PAGE_SIZE},
    errors::HttpError,
    extractors::users::AuthenticatedUserClaims,
    shared_app_data::SharedAppData,
};

#[derive(Deserialize, ToSchema, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct GetAnnouncementsListRequestQueries {
    /// page number of the data to query. Default is `1`
    #[param(default = json!(1))]
    pub page: Option<i32>,
    /// how big of the page to query. Default is `10`
    #[param(default = json!(crate::constants::DEFAULT_PAGE_SIZE))]
    pub page_size: Option<i32>,
}

#[derive(Serialize, ToSchema)]
pub struct GetAnnouncementsListResponseBody {
    pub announcements: Vec<GetAnnouncementsListResponseBodyInner>,
}

impl Default for GetAnnouncementsListResponseBody {
    fn default() -> Self {
        Self {
            announcements: vec![],
        }
    }
}

#[derive(FromRow, Serialize, ToSchema)]
pub struct GetAnnouncementsListResponseBodyInner {
    pub announcement_id: String,
    pub announcement_name: String,
    pub user_id: String,
    pub announcement_content: String,
    pub announcement_created_timestamp: time::OffsetDateTime,
    pub announcement_view_amount: u32,
    pub announcement_reply_amount: u32,
}

/// Get global announcements to be shown on first page.
#[utoipa::path(
    get,
    path = "/forum/announcements",
    tag = "forum",
    operation_id = "get_announcements_list",
    params(AuthenticationHeaders),
    responses(
        (
            status = 200,
            description = "successfully get list of forums",
            body = GetAnnouncementsListResponseBody,
            example = json!(GetAnnouncementsListResponseBody::default())
        ),
    )
)]
pub async fn handler(
    query: web::Query<GetAnnouncementsListRequestQueries>,
    _claims: AuthenticatedUserClaims,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    let client = data.pool.get().await?;

    let page = query.page.unwrap_or(DEFAULT_PAGE);
    let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

    let statement = client.prepare_typed(
        r##"
        select
            forum_global_announcements.forum_global_announcement_id as announcement_id,
            forum_global_announcements.forum_global_announcement_name as announcement_name,
            forum_global_announcements.user_id as user_id,
            forum_global_announcements.forum_global_announcement_content as announcement_content,
            forum_global_announcements.forum_global_announcement_created_timestamp as announcement_created_timestamp,
            count(forum_post_views.forum_post_id) as announcement_view_amount,
            count(forum_post_replies.forum_post_id) as announcement_reply_amount
        from forum_global_announcements
        inner join forum_post_views on forum_global_announcements.forum_global_announcement_id = forum_post_views.forum_post_id
        inner join forum_post_replies on forum_global_announcements.forum_global_announcement_id = forum_post_replies.forum_post_id
        where forum_global_announcements.forum_global_announcement_is_active = true
        limit $1
        offset $2
        "##,
        &[Type::INT4, Type::INT4],
    ).await?;

    let rows = client
        .query(&statement, &[&page_size, &((page * page_size) - page_size)])
        .await?;

    let announcements: Vec<GetAnnouncementsListResponseBodyInner> = rows
        .iter()
        .map(|r| return GetAnnouncementsListResponseBodyInner::try_from(r))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| HttpError::InternalServerError)?;

    let response = HttpResponse::Ok().json(GetAnnouncementsListResponseBody { announcements });
    Ok(response)
}
