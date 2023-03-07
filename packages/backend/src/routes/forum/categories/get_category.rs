use actix_web::{web, HttpResponse};
use ger_from_row::FromRow;
use postgres_types::Type;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use utoipa::{IntoParams, ToSchema};

use crate::{errors::HttpError, shared_app_data::SharedAppData};

#[derive(Deserialize, ToSchema, IntoParams, TS)]
#[ts(export)]
#[into_params(parameter_in = Path)]
pub struct GetCategoryRequestParams {
    pub category_representative_id: String,
}

#[derive(Serialize, Deserialize, ToSchema, TS, FromRow)]
#[ts(export)]
pub struct GetCategoryResponseBody {
    id: String,
    name: String,
    representative_id: String,
    description: String,
    color_theme: String,
}

impl Default for GetCategoryResponseBody {
    fn default() -> Self {
        Self {
            id: "ND_XNQyNtAIUPGAf4kHES0MhfPmXwKrC".to_string(),
            name: "Homeworks".to_string(),
            representative_id: "homeworks".to_string(),
            description: "Homework related questions.".to_string(),
            color_theme: "#223344".to_string(),
        }
    }
}

#[utoipa::path(
    get,
    path = "/forum/categories/{category_representative_id}",
    tag = "forum",
    operation_id = "get_category",
    params(GetCategoryRequestParams),
    responses(
        (
            status = 200,
            description = "successfully get category metadata",
            body = GetCategoryResponseBody,
            example = json!(GetCategoryResponseBody::default())
        ),
        (
            status = 400,
            description = "input errors",
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
            status = 404,
            description = "not found",
            body = FormattedErrorResponse,
            example = json!(HttpError::CategoryNotFound.get_error_struct())
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
    params: web::Path<GetCategoryRequestParams>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    let client = data.pool.get().await?;

    let statement = client
        .prepare_typed_cached(
            r##"
            select
                forum_category_id as id,
                forum_category_name as name,
                forum_category_representative_id as representative_id,
                forum_category_description as description,
                forum_category_color_theme as color_theme
            from forum_categories
            where forum_category_representative_id = $1
            "##,
            &[Type::TEXT],
        )
        .await?;

    let category = client
        .query_one(&statement, &[&params.category_representative_id])
        .await
        .map_err(|e| {
            tracing::error!("{}", e);
            HttpError::CategoryNotFound
        })?;

    let category = GetCategoryResponseBody::try_from(&category)?;
    Ok(HttpResponse::Ok().json(category))
}
