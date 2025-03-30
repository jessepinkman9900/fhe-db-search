mod config;
mod db;
mod router;

#[tokio::main]
async fn main() {
    env_logger::init();
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
