use deadpool_postgres::Config;

mod routes;

#[actix_web::main]
async fn main() {
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
    postgres_config.port =
        Some(u16::try_from(postgres_port).expect("cannot convert postgres port to u16"));
    postgres_config.dbname = Some(postgres_database_name);
}
