use axum::{
    routing::{get, post},
    http::StatusCode,
    response::{Html, Json},
    Router,
};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::env;

use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use chrono::Utc;

use html_pages::home_page;
use pages_components::ButtonComponent;


#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,my_ssr_server=debug,tower_http=debug,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("Server application starting...");

    let current_exe_path = env::current_exe()
        .expect("Failed to get current executable path");

    let current_dir = current_exe_path.parent()
        .expect("Failed to get parent directory of executable");

    let static_files_path = current_dir.join("static");

    tracing::info!("Serving static files from: {:?}", static_files_path);


    let app = Router::new()
        .route("/", get(index_handler))
        .route("/api/data", get(api_data_handler))
        .fallback_service(ServeDir::new(static_files_path));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

async fn index_handler() -> Html<String> {
    tracing::info!("Received request for /");

    let page_level_data = vec![
        String::from("<p>This is dynamic page-level data.</p>"),
        String::from("<p>More data here!</p>"),
    ];

    let css_files_markup = vec![
        String::from(r#"<link rel="stylesheet" href="global.css">"#),
    ];

    let components_markup = vec![
        String::from(r#"<div class="some-other-component">This is a generic component placeholder!</div>"#),
    ];

    let js_files_markup = vec![
        String::from(r#"<script src="/home_page.js"></script>"#),
    ];

    let full_html = html_pages::home_page::print_page(
        &page_level_data,
        &css_files_markup,
        &components_markup,
        &js_files_markup,
    );

    Html(full_html)
}

async fn api_data_handler() -> Json<serde_json::Value> {
    tracing::info!("Received request for /api/data");

    Json(serde_json::json!({
        "message": "Hello from the Rust API!",
        "timestamp": Utc::now().to_rfc3339(), // `Utc` is now correctly imported
        "data": {
            "key1": "value1",
            "key2": 123
        }
    }))
}