use std::{fs::File, io::BufReader};

use actix_web::{web, App, HttpServer};
use deadpool_postgres::Config;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use tracing_actix_web::TracingLogger;

mod constants;
mod errors;
mod openapi;
mod routes;

/// Load key file and certificates file for spinnning server up in https context
fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth();

    let key_file =
        &mut BufReader::new(File::open("cert/ger-key.key").expect("missing tls key file"));
    let cert_file =
        &mut BufReader::new(File::open("cert/ger-cert.pem").expect("missing tls certificate file"));

    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();
    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    if keys.is_empty() {
        eprintln!("could not locate pkcs 8 private keys");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // environment variables setup
    dotenvy::from_filename(".env.local").expect("no environment variables file found");

    // postgres related setup
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

    // logging setup
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(non_blocking_writer)
        .init();

    // https setup
    let rustls_config = load_rustls_config();

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(crate::routes::hello::handler))
    })
    .bind_rustls(("127.0.0.1", 5155), rustls_config)
    .expect("cannot start https server")
    .run()
    .await
}
