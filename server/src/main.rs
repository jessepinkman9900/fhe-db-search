mod config;
mod db;
mod router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_level(true)
                .with_file(true)
                .with_line_number(true)
                .compact(),
        )
        .init();

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
