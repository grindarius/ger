use actix_web::{web, HttpResponse};
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use rand_core::OsRng;
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio_postgres::types::Type;
use utoipa::ToSchema;

use crate::constants::{create_argon2_context, responses::DefaultSuccessResponse, ID_LENGTH};
use crate::constants::{swagger::AuthenticationHeaders, AD_BE_YEAR_DIFFERENCE};
use crate::database::Role;
use crate::errors::HttpError;
use crate::extractors::admins::AuthenticatedAdminClaims;
use crate::shared_app_data::SharedAppData;

#[derive(Deserialize, ToSchema)]
pub struct StudentSignupRequestBody {
    students: Vec<StudentSignupRequestBodyInner>,
    major_id: String,
    #[schema(example = json!("31"))]
    major_representative_id: String,
    professor_id: String,
    /// academic year id of student that will be recognized.
    first_academic_year_id: String,
}

#[derive(Deserialize, ToSchema)]
pub struct StudentSignupRequestBodyInner {
    #[schema(example = json!("8365079019452"))]
    student_nid: String,
    student_english_first_name: String,
    student_english_middle_name: String,
    student_english_last_name: String,
    #[schema(value_type = String, format = Date)]
    #[serde(with = "time::Date")]
    student_birthdate: time::Date,
    student_previous_school_name: String,
    #[serde(with = "rust_decimal::serde::float")]
    #[schema(value_type = f32, example = json!(3.99))]
    student_previous_school_gpa: Decimal,
}

#[derive(Deserialize, ger_from_row::FromRow)]
struct LatestStudentIndex {
    student_representative_id: String,
}

/// Bulk signup students either given from a csv file or some admission website.
#[utoipa::path(
    post,
    path = "/students/signup",
    tag = "students",
    operation_id = "signup",
    params(AuthenticationHeaders),
    request_body = StudentSignupRequestBody,
    responses(
        (
            status = 200,
            description = "successfully added students",
            body = DefaultSuccessResponse,
            example = json!(DefaultSuccessResponse::default())
        ),
        (
            status = 400,
            description = "input errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InputValidationError.get_error_struct())
        ),
        (
            status = 401,
            description = "unauthorized error",
            body = FormattedErrorResponse,
            example = json!(HttpError::Unauthorized.get_error_struct())
        ),
        (
            status = 500,
            description = "internal server errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InternalServerError.get_error_struct())
        )
    )
)]
pub async fn handler(
    body: web::Json<StudentSignupRequestBody>,
    data: web::Data<SharedAppData>,
    _claims: AuthenticatedAdminClaims,
) -> Result<HttpResponse, HttpError> {
    if body.students.len() == 0 {
        return Err(HttpError::NoData);
    }

    let mut client = data.pool.get().await?;

    let get_numeric_first_academic_year_statement = client
        .prepare_typed(
            r##"
            select
                academic_year_id,
                academic_year_anno_domini_year
            from academic_years
            where academic_year_id = $1
            "##,
            &[Type::TEXT],
        )
        .await?;

    let first_academic_year = client
        .query_one(
            &get_numeric_first_academic_year_statement,
            &[&body.first_academic_year_id],
        )
        .await?;
    let first_academic_year = first_academic_year
        .try_get::<usize, i32>(1usize)
        .map_err(|_| HttpError::InternalServerError)?;

    let first_academic_year_bhuddist_era_year = first_academic_year + AD_BE_YEAR_DIFFERENCE as i32;

    let insert_user_statement = client
        .prepare_typed_cached(
            r##"
            insert into users (
                user_id,
                user_username,
                user_email,
                user_password,
                user_role,
                user_birthdate
            ) values (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            )"##,
            &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::DATE,
            ],
        )
        .await?;

    let insert_student_statement = client
        .prepare_typed_cached(
            r##"
            insert into students (
                student_id,
                student_representative_id,
                student_nid,
                student_previous_school_name,
                student_previous_school_gpa,
                major_id,
                professor_id,
                first_academic_year_id
            ) values (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8
            )"##,
            &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::NUMERIC,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
            ],
        )
        .await?;

    let insert_student_name_statement = client
        .prepare_typed_cached(
            r##"
            insert into student_names (
                student_name_id,
                student_id,
                student_name_language,
                student_firstname,
                student_middlename,
                student_lastname
            ) values (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6
            )"##,
            &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
            ],
        )
        .await?;

    let argon2_context = create_argon2_context()?;

    let latest_student_index = client
        .query_one(
            r##"
            select
                student_representative_id
            from students
            where major_id = $1 and first_academic_year_id = $2"##,
            &[&body.major_id, &body.first_academic_year_id],
        )
        .await?;
    let latest_student_index = LatestStudentIndex::try_from(latest_student_index)?;
    let latest_student_number = latest_student_index.student_representative_id[4..]
        .parse::<usize>()
        .unwrap_or(0usize);

    for (i, student) in body.students.iter().enumerate() {
        let transaction = client.transaction().await?;

        let new_student_id = nanoid::nanoid!(ID_LENGTH);
        let new_student_email = format!(
            "{}{}{}@gmail.com",
            &student.student_english_first_name,
            &student
                .student_english_last_name
                .as_str()
                .chars()
                .nth(0)
                .unwrap_or('s')
                .to_string(),
            &first_academic_year_bhuddist_era_year,
        );
        let new_student_index = latest_student_number + i;

        if new_student_index == 10000 {
            tracing::error!("new student_index overflow to {}", new_student_index);
            return Err(HttpError::InternalServerError);
        }

        let new_student_representative_id = format!(
            "{}{}{:0>4}",
            first_academic_year_bhuddist_era_year, body.major_representative_id, new_student_index
        );
        let salt = SaltString::generate(&mut OsRng);
        let new_student_account_password =
            argon2_context.hash_password(&new_student_representative_id.as_bytes(), &salt)?;

        transaction
            .execute(
                &insert_user_statement,
                &[
                    &new_student_id,
                    &nanoid::nanoid!(10),
                    &new_student_email,
                    &new_student_account_password.to_string(),
                    &Role::Student,
                    &student.student_birthdate,
                ],
            )
            .await?;
        transaction
            .execute(
                &insert_student_statement,
                &[
                    &new_student_id,
                    &new_student_representative_id,
                    &student.student_nid,
                    &student.student_previous_school_name,
                    &student.student_previous_school_gpa,
                    &body.major_id,
                    &body.professor_id,
                    &body.first_academic_year_id,
                ],
            )
            .await?;
        transaction
            .execute(
                &insert_student_name_statement,
                &[
                    &nanoid::nanoid!(ID_LENGTH),
                    &new_student_id,
                    &"EN",
                    &student.student_english_first_name.trim(),
                    &student.student_english_middle_name.trim(),
                    &student.student_english_last_name.trim(),
                ],
            )
            .await?;

        transaction.commit().await?;
    }

    Ok(HttpResponse::Ok().json(DefaultSuccessResponse::default()))
}
