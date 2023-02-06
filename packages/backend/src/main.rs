use std::{fs::File, io::BufReader};

use actix_web::{web, App, HttpServer, ResponseError};
use constants::APP_NAME;
use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
use opentelemetry::global;
use opentelemetry::runtime::Tokio;
use opentelemetry::sdk::propagation::TraceContextPropagator;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tokio_postgres::NoTls;
use tracing_actix_web::TracingLogger;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::openapi::apidoc::ApiDoc;
use crate::shared_app_data::SharedAppData;

mod constants;
mod database;
mod errors;
mod extractors;
mod openapi;
mod routes;
mod shared_app_data;

/// Load key file and certificates file for spinnning server up in https context
fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let key_file = &mut BufReader::new(
        File::open("packages/backend/cert/ger-key.key").expect("missing tls key file"),
    );
    let cert_file = &mut BufReader::new(
        File::open("packages/backend/cert/ger-cert.pem").expect("missing tls certificate file"),
    );

    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .expect("cannot load private key file")
        .into_iter()
        .map(PrivateKey)
        .collect();
    let cert_chain = certs(cert_file)
        .expect("cannot load certificate file")
        .into_iter()
        .map(Certificate)
        .collect();

    if keys.is_empty() {
        panic!("could not locate pkcs 8 private keys");
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

/// Load postgres config from environment variables
fn load_postgres_config() -> Config {
    let postgres_username =
        dotenvy::var("GER_POSTGRES_USERNAME").expect("missing postgres username");
    let postgres_password =
        dotenvy::var("GER_POSTGRES_PASSWORD").expect("missing postgres password");
    let postgres_host_ip = dotenvy::var("GER_POSTGRES_HOST").expect("missing postgres host ip");
    let postgres_port = dotenvy::var("GER_POSTGRES_PORT").expect("missing postgres port");
    let postgres_database_name =
        dotenvy::var("GER_POSTGRES_DATABASE_NAME").expect("missing postgres database name");

    let mut postgres_config = Config::new();
    postgres_config.user = Some(postgres_username);
    postgres_config.password = Some(postgres_password);
    postgres_config.host = Some(postgres_host_ip);
    postgres_config.port = Some(
        postgres_port
            .parse::<u16>()
            .expect("cannot convert postgres port to u16"),
    );
    postgres_config.dbname = Some(postgres_database_name);
    postgres_config.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    postgres_config
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // environment variables setup
    dotenvy::from_filename(".env.local").expect("no environment variables file found");

    // postgres related setup
    let postgres_config = load_postgres_config();
    let pool = postgres_config
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("cannot create postgres pool from a given config");

    // https setup
    let rustls_config = load_rustls_config();

    // log setup
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(APP_NAME)
        .install_batch(Tokio)
        .expect("failed to install Opentelemetry tracer");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let bunyan_formatter = BunyanFormattingLayer::new(APP_NAME.into(), non_blocking_writer);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(bunyan_formatter);

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to install tracing subscriber");

    // openapi setup
    let openapi = ApiDoc::openapi();

    tracing::info!("starting https server at https://127.0.0.1:5155");
    tracing::info!("starting swagger ui at https://127.0.0.1:5155/swagger-doc/");

    HttpServer::new(move || {
        // cors config
        let cors = actix_cors::Cors::default()
            .allowed_origin("https://127.0.0.1")
            .supports_credentials();

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
                "/students/signup",
                web::post().to(crate::routes::students::signup::handler),
            )
            .route(
                "/forum/announcements",
                web::get().to(crate::routes::forum::announcements::get_announcements_list::handler),
            )
            .route(
                "/forum/trending",
                web::get().to(crate::routes::forum::get_trending_posts_list::handler),
            )
            .service(
                SwaggerUi::new("/swagger-doc/{_:.*}").url("/openapi/openapi.json", openapi.clone()),
            )
    })
    .bind_rustls(("127.0.0.1", 5155), rustls_config)
    .expect("cannot start https server")
    .run()
    .await
}
