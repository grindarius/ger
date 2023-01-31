use actix_web::{web, HttpResponse};
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
use rand_core::OsRng;
use rust_decimal::Decimal;
use serde::Deserialize;
use tokio_postgres::types::Type;
use utoipa::ToSchema;

use crate::constants::AuthenticationHeaders;
use crate::constants::{create_argon2_context, DefaultSuccessResponse, ID_LENGTH};
use crate::database::Role;
use crate::errors::HttpError;
use crate::extractors::admins::AuthenticatedAdminClaims;
use crate::shared_app_data::SharedAppData;

#[derive(Deserialize, ToSchema)]
pub struct StudentSignupBody {
    students: Vec<StudentSignupBodyInner>,
    major_id: String,
    #[schema(example = json!("31"))]
    major_representative_id: String,
    professor_id: String,
    first_academic_year_id: String,
    #[schema(example = json!("2017"))]
    first_academic_year_gregorian_year: String,
}

#[derive(Deserialize, ToSchema)]
pub struct StudentSignupBodyInner {
    #[schema(example = json!("8365079019452"))]
    student_nid: String,
    student_english_first_name: String,
    student_english_middle_name: String,
    student_english_last_name: String,
    #[schema(value_type = String, format = Date)]
    #[serde(with = "time::Date")]
    student_birthdate: time::Date,
    student_previous_school_name: String,
    #[serde(with = "rust_decimal::serde::str")]
    #[schema(example = json!("3.99"))]
    student_previous_school_gpa: Decimal,
}

#[derive(Deserialize, ger_from_row::FromRow)]
struct LatestStudentIndex {
    latest_student_index: u32,
}

/// Bulk signup students either given from a csv file or some admission website.
#[utoipa::path(
    post,
    path = "/students/signup",
    request_body = StudentSignupBody,
    params(AuthenticationHeaders),
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
            status = 500,
            description = "internal server errors",
            body = FormattedErrorResponse,
            example = json!(HttpError::InternalServerError.get_error_struct())
        )
    )
)]
pub async fn handler(
    body: web::Json<StudentSignupBody>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    if body.students.len() == 0 {
        return Err(HttpError::NoData);
    }

    let mut client = data.pool.get().await?;
    let first_academic_year_anno_domini_year = body
        .first_academic_year_gregorian_year
        .parse::<u32>()
        .map_err(|_| HttpError::InputValidationError)?
        + 543u32;

    let insert_user_statement = client
        .prepare_typed_cached(
            r##"
            insert into users (
                user_id,
                user_username,
                user_email,
                user_password,
                user_role
            ) values (
                $1,
                $2,
                $3,
                $4,
                $5
            )"##,
            &[Type::TEXT, Type::TEXT, Type::TEXT, Type::TEXT, Type::TEXT],
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

    let insert_student_statement = client
        .prepare_typed_cached(
            r##"insert into students (
                    student_id,
                    student_representative_id,
                    student_nid,
                    student_birthdate,
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
                    $8,
                    $9
                )"##,
            &[
                Type::TEXT,
                Type::TEXT,
                Type::TEXT,
                Type::DATE,
                Type::TEXT,
                Type::NUMERIC,
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
                count(student_id) as latest_student_index 
            from students
            where major_id = $1 and first_academic_year_id = $2"##,
            &[&body.major_id, &body.first_academic_year_id],
        )
        .await?;
    let latest_student_index = LatestStudentIndex::try_from(latest_student_index)?;

    for (i, student) in body.students.iter().enumerate() {
        let transaction = client.transaction().await.unwrap();

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
            &first_academic_year_anno_domini_year,
        );
        let new_student_major_index = latest_student_index.latest_student_index as usize + i;
        let new_student_representative_id = format!(
            "{}{}{:0>4}",
            first_academic_year_anno_domini_year,
            body.major_representative_id,
            new_student_major_index
        );
        let salt = SaltString::generate(&mut OsRng);
        let new_student_account_password = create_argon2_context()?
            .hash_password(&new_student_representative_id.as_bytes(), &salt)?;

        transaction
            .execute(
                &insert_user_statement,
                &[
                    &new_student_id,
                    &nanoid::nanoid!(10),
                    &new_student_email,
                    &new_student_account_password.to_string(),
                    &Role::Student.as_str(),
                ],
            )
            .await?;

        transaction.execute(&insert_student_statement, &[]).await?;

        transaction
            .execute(&insert_student_name_statement, &[])
            .await?;

        transaction.commit().await?;
    }

    Ok(HttpResponse::Ok().json(DefaultSuccessResponse::default()))
}
