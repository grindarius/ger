use actix_web::{web, HttpResponse};
use argon2::{password_hash::SaltString, PasswordHasher};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    constants::{
        create_argon2_context, responses::DefaultSuccessResponse, ARGON2_PEPPER_STRING, ID_LENGTH,
    },
    database::Role,
    errors::HttpError,
    shared_app_data::SharedAppData,
};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AdminSignupRequestBody {
    pub username: String,
    pub email: String,
    pub password: String,
    #[schema(value_type = String, format = Date)]
    #[serde(with = "time::Date")]
    pub birthdate: time::Date,
}

#[utoipa::path(
    post,
    path = "/admin/signup",
    tag = "admin",
    operation_id = "signup",
    request_body = AdminSignupRequestBody,
    responses(
        (
            status = 200,
            description = "new admin created",
            body = DefaultSuccessResponse,
            example = json!(DefaultSuccessResponse::default())
        ),
    )
)]
pub async fn handler(
    body: web::Json<AdminSignupRequestBody>,
    data: web::Data<SharedAppData>,
) -> Result<HttpResponse, HttpError> {
    if body.username.is_empty() {
        return Err(HttpError::InputValidationError);
    }

    if body.email.is_empty() {
        return Err(HttpError::InputValidationError);
    }

    if body.password.is_empty() {
        return Err(HttpError::InputValidationError);
    }

    let client = data.pool.get().await?;

    let possible_redundancies = client
        .query(
            "select user_id from users where user_username = $1 or user_email = $2",
            &[&body.username, &body.email],
        )
        .await?;

    if possible_redundancies.len() > 0 {
        return Err(HttpError::InputValidationError);
    }

    let statement = client
        .prepare(
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
            )
            "##,
        )
        .await?;

    let context = create_argon2_context(&ARGON2_PEPPER_STRING)?;
    let salt = SaltString::generate(&mut OsRng);
    let password = context.hash_password(body.password.as_bytes(), salt.as_str())?;

    client
        .execute(
            &statement,
            &[
                &randoid::randoid!(ID_LENGTH),
                &body.username,
                &body.email,
                &password.to_string(),
                &Role::Admin,
                &body.birthdate,
            ],
        )
        .await?;

    Ok(HttpResponse::Created().json(DefaultSuccessResponse::default()))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, test, web, App};
    use deadpool_postgres::Runtime;
    use serde_json::json;
    use tokio_postgres::NoTls;

    use crate::{shared_app_data::SharedAppData, startup::load_postgres_config};

    use super::{handler, AdminSignupRequestBody};

    #[actix_web::test]
    async fn test_admin_signup() {
        let postgres_config = load_postgres_config();
        let pool = postgres_config
            .create_pool(Some(Runtime::Tokio1), NoTls)
            .unwrap();
        let client = pool.get().await.unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(SharedAppData::new(pool.clone())))
                .route("/", web::post().to(handler)),
        )
        .await;

        let username = "grindarius";
        let email = "grindarius@gmail.com";
        let password = "grindarius";
        let birthdate =
            time::OffsetDateTime::now_utc() - time::Duration::new(60 * 60 * 24 * 365 * 30, 0);
        let birthdate = birthdate.date();

        client
            .execute("delete from users where user_username = $1", &[&username])
            .await
            .unwrap();

        // empty username
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(AdminSignupRequestBody {
                username: "".to_string(),
                email: email.to_string(),
                password: password.to_string(),
                birthdate,
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // empty email
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(AdminSignupRequestBody {
                username: username.to_string(),
                email: "".to_string(),
                password: password.to_string(),
                birthdate,
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // empty password
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(AdminSignupRequestBody {
                username: username.to_string(),
                email: email.to_string(),
                password: "".to_string(),
                birthdate,
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // empty birthdate
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(json!({
                "username": username,
                "email": email,
                "password": password,
                "birthdate": ""
            }))
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        // successful call
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(AdminSignupRequestBody {
                username: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
                birthdate,
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::CREATED);

        // redundant username
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(AdminSignupRequestBody {
                username: username.to_string(),
                email: email.to_string(),
                password: password.to_string(),
                birthdate,
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        // redundant email
        let request = test::TestRequest::post()
            .uri("/")
            .set_json(AdminSignupRequestBody {
                username: "grindarius2".to_string(),
                email: email.to_string(),
                password: password.to_string(),
                birthdate,
            })
            .to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
