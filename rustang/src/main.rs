use axum::{
    routing::get,
    Router,
    Json,
    response::Json as AxumJson,
    serve,
};
use serde_json::json;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

// Handler cho health check
async fn health_check() -> AxumJson<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "rustang",
        "version": "0.1.0"
    }))
}

// Handler cho GET /
async fn root_handler() -> &'static str {
    "Rustang API Server is running! 🚀"
}

#[tokio::main]
async fn main() {
    // init logging
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    // load variables from enviroment
    dotenvy::dotenv().ok();

    // extract port
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    // build router
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive());

    // socket address
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("🚀 Rustang server running on http://{}", addr);

    // init listener and run server
    let listener = TcpListener::bind(&addr).await.unwrap();
    serve(listener, app).await.unwrap();
}