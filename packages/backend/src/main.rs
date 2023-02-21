use actix_web::{web, App, HttpServer, ResponseError};
use deadpool_postgres::Runtime;
use tokio_postgres::NoTls;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{openapi::apidoc::ApiDoc, shared_app_data::SharedAppData, startup::*};

mod constants;
mod database;
mod errors;
mod extractors;
mod openapi;
mod routes;
mod shared_app_data;
mod startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // environment variables setup
    dotenvy::from_filename(".env.local").expect("no environment variables file found");

    // postgres setup
    let postgres_config = load_postgres_config();
    let pool = postgres_config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("cannot create postgres pool from a given config");

    // log setup, guard has to stay there, cannot be dropped, if dropped could result in weird
    // behavior of logging.
    let _guard = init_telemetry();

    // openapi setup
    let openapi = ApiDoc::openapi();

    tracing::info!("starting https server at http://127.0.0.1:5155");
    tracing::info!("starting swagger ui at http://127.0.0.1:5155/swagger-doc/");

    HttpServer::new(move || {
        // cors config
        let cors = actix_cors::Cors::permissive();

        // deserializer errors config
        let json_deserialize_config =
            web::JsonConfig::default().error_handler(|error, _request| {
                let error_message = Clone::clone(&error.to_string());
                let status_code = Clone::clone(&error.status_code());

                actix_web::error::InternalError::from_response(
                    error,
                    actix_web::HttpResponse::build(status_code).json(
                        crate::errors::FormattedErrorResponse {
                            status_code: status_code.as_u16(),
                            error: "json deserialize error".to_string(),
                            message: error_message,
                        },
                    ),
                )
                .into()
            });

        let path_deserialize_config =
            web::PathConfig::default().error_handler(|error, _request| {
                let error_message = Clone::clone(&error.to_string());
                let status_code = Clone::clone(&error.status_code());

                actix_web::error::InternalError::from_response(
                    error,
                    actix_web::HttpResponse::build(status_code).json(
                        crate::errors::FormattedErrorResponse {
                            status_code: status_code.as_u16(),
                            error: "path deserialize error".to_string(),
                            message: error_message,
                        },
                    ),
                )
                .into()
            });

        let query_deserialize_config =
            web::QueryConfig::default().error_handler(|error, _request| {
                let error_message = Clone::clone(&error.to_string());
                let status_code = Clone::clone(&error.status_code());

                actix_web::error::InternalError::from_response(
                    error,
                    actix_web::HttpResponse::build(status_code).json(
                        crate::errors::FormattedErrorResponse {
                            status_code: status_code.as_u16(),
                            error: "query deserialize error".to_string(),
                            message: error_message,
                        },
                    ),
                )
                .into()
            });

        App::new()
            .app_data(web::Data::new(SharedAppData::new(pool.clone())))
            .app_data(json_deserialize_config)
            .app_data(path_deserialize_config)
            .app_data(query_deserialize_config)
            .wrap(TracingLogger::default())
            .wrap(cors)
            .route("/", web::get().to(crate::routes::hello::handler))
            .route(
                "/auth/signin",
                web::post().to(crate::routes::auth::signin::handler),
            )
            .route(
                "/auth/refresh",
                web::post().to(crate::routes::auth::refresh::handler),
            )
            .route(
                "/admin/signup",
                web::post().to(crate::routes::admin::signup::handler),
            )
            .route(
                "/users",
                web::get().to(crate::routes::users::get_users_list::handler),
            )
            .route(
                "/users/{user_id}/profile-image",
                web::get().to(crate::routes::users::get_user_profile_image::handler),
            )
            .route(
                "/students/signup",
                web::post().to(crate::routes::students::signup::handler),
            )
            .route(
                "/forum/posts",
                web::get().to(crate::routes::forum::posts::get_post_list::handler),
            )
            .route(
                "/forum/posts/trending",
                web::get().to(crate::routes::forum::posts::get_trending_posts_list::handler),
            )
            .route(
                "/forum/posts/{post_id}",
                web::get().to(crate::routes::forum::posts::get_post::handler),
            )
            .route(
                "/forum/posts/{post_id}/replies",
                web::get().to(crate::routes::forum::posts::get_post_replies::handler),
            )
            .route(
                "/forum/categories",
                web::get().to(crate::routes::forum::categories::get_categories_list::handler),
            )
            .service(
                SwaggerUi::new("/swagger-doc/{_:.*}").url("/openapi/openapi.json", openapi.clone()),
            )
    })
    .bind((
        "127.0.0.1",
        dotenvy::var("GER_API_PORT")
            .expect("cannot find GER_API_PORT environment variable")
            .parse::<u16>()
            .expect("cannot parse port environment variable"),
    ))
    .expect("cannot start http server")
    .run()
    .await
}
