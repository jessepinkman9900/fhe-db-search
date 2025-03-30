use crate::db;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::http::HeaderName;
use axum::http::Request;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::routing::post;
use axum::{Json, Router};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::PgConnection;
use serde_json;
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::{error, info, info_span};

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

pub fn create_router(state: AppState) -> Router {
    const REQUEST_ID_HEADER: &str = "x-request-id";
    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(
            x_request_id.clone(),
            MakeRequestUuid,
        ))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the request id as generated.
                let request_id = request.headers().get(REQUEST_ID_HEADER);

                match request_id {
                    Some(request_id) => info_span!(
                        "http_request",
                        request_id = ?request_id,
                        method = ?request.method(),
                        uri = ?request.uri(),
                    ),
                    None => {
                        error!("could not extract request_id");
                        info_span!("http_request")
                    }
                }
            }),
        )
        // send headers from request to response headers
        .layer(PropagateRequestIdLayer::new(x_request_id));

    Router::new()
        .route("/health", get(health_check_handler))
        .route("/kv", post(create_kv_handler))
        .route("/kvs", get(get_kvs_handler))
        .layer(middleware)
        .with_state(state)
}

mod header_keys {
    pub const X_CLIENT_ID: &str = "X-Client-ID";
}

async fn health_check_handler() -> impl IntoResponse {
    const MSG: &str = "UP";

    let json_response = serde_json::json!({
      "status": "success",
      "message": MSG
    });

    Json(json_response)
}

async fn create_kv_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    // input validation
    let client_id = headers
        .get(header_keys::X_CLIENT_ID)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let key = payload["key"].as_str().unwrap().to_string();
    let value = payload["value"].as_str().unwrap().to_string();

    // perform command
    let conn = &mut db::get_db_connection(&state.pool).await.unwrap();
    let kv = db::create_kv(conn, client_id, key, value).await;
    Json(kv)
}

async fn get_kvs_handler(State(state): State<AppState>) -> impl IntoResponse {
    let conn = &mut db::get_db_connection(&state.pool).await.unwrap();
    let kvs = db::get_kvs(conn).await;
    Json(kvs)
}
