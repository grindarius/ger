use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod};
use opentelemetry::{global, runtime::Tokio, sdk::propagation::TraceContextPropagator};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};

use crate::constants::APP_NAME;

/// Load postgres config from environment variables
pub fn load_postgres_config() -> Config {
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

/// initialize telemetry settings
pub fn init_telemetry() -> WorkerGuard {
    global::set_text_map_propagator(TraceContextPropagator::new());
    let tracer = opentelemetry_jaeger::new_agent_pipeline()
        .with_service_name(APP_NAME)
        .install_batch(Tokio)
        .expect("failed to install Opentelemetry tracer");

    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let (non_blocking_writer, guard) = tracing_appender::non_blocking(std::io::stdout());
    let bunyan_formatter = BunyanFormattingLayer::new(APP_NAME.into(), non_blocking_writer);

    let subscriber = Registry::default()
        .with(env_filter)
        .with(telemetry)
        .with(JsonStorageLayer)
        .with(bunyan_formatter);

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to install tracing subscriber");

    guard
}
