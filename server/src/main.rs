mod config;
mod db;
mod router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // env_logger::init();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
                .compact()
                .with_file(false)
                .with_line_number(false)
                .with_thread_ids(true)
                .with_ansi(true),
        )
        .init();
    // let subscriber = tracing_subscriber::fmt().finish();

    // // Set the subscriber as the global default
    // tracing::subscriber::set_global_default(subscriber).unwrap();
    log::info!("Starting server");

    let config = config::load_config().unwrap();

    // postgres client
    let pg_connection_pool = db::connect_db(&config);
    log::info!("Connected to database");

    let app_state = router::AppState {
        pool: pg_connection_pool,
    };

    log::info!("Starting server");
    let app = router::create_router(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
