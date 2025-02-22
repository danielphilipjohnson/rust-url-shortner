use axum::body::to_bytes;
use axum::{
    body::{Body, Bytes},
    extract::Request,
    http::StatusCode,
    response::Response,
    routing::{get, post},
    Router,
};
use mongodb::{options::ClientOptions, Client};
use serde_json::{json, Value};
use std::env;
use std::sync::Arc;
use tower::ServiceExt;

use url_shortener::{
    handlers::{
        health::{health_routes, init_metrics},
        url_handler::UrlHandler,
    },
    services::url_service::UrlService,
};

use std::time::Duration;
use tokio::time::sleep;

async fn wait_for_mongodb(db_uri: &str) {
    for _ in 0..5 {
        match Client::with_uri_str(db_uri).await {
            Ok(client) => {
                if client
                    .database("admin")
                    .run_command(mongodb::bson::doc! {"ping": 1})
                    .await
                    .is_ok()
                {
                    println!("✅ MongoDB is ready!");
                    return;
                }
            }
            Err(_) => {
                println!("⏳ Waiting for MongoDB...");
                sleep(Duration::from_secs(3)).await;
            }
        }
    }
    panic!("❌ MongoDB did not start in time");
}

const BODY_LIMIT: usize = 16 * 1024;

async fn setup_test_app() -> Router {
    dotenv::dotenv().ok();

    let db_uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
        "mongodb://admin:secret@localhost:27017/url_shortener_test?authSource=admin".to_string()
    });

    wait_for_mongodb(&db_uri).await;

    let client_options = ClientOptions::parse(&db_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    let db = client.database("url_shortener_test");
    let collection = db.collection("urls");

    // Clean the test database before each test
    collection.drop().await.unwrap();

    let url_service = UrlService::new(collection);
    let url_handler = Arc::new(UrlHandler::new(url_service));

    let create_handler: Arc<UrlHandler> = Arc::clone(&url_handler);
    let redirect_handler: Arc<UrlHandler> = Arc::clone(&url_handler);
    let get_all_handler: Arc<UrlHandler> = Arc::clone(&url_handler);

    Router::new()
        .route("/", get(|| async { "✅ URL Shortener is running!" }))
        .merge(health_routes())
        .route(
            "/api/shorturl",
            post(move |payload| async move { create_handler.create_short_url(payload).await }),
        )
        .route(
            "/api/shorturl/{short_url{}",
            get(move |path| async move { redirect_handler.redirect_to_original(path).await }),
        )
        .route(
            "/api/shorturls",
            get(move || async move { get_all_handler.get_all_short_urls().await }),
        )
}

async fn read_body(body: Body) -> Result<Value, String> {
    let bytes = to_bytes(body, BODY_LIMIT)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

#[tokio::test]
async fn test_health_endpoint() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = read_body(response.into_body()).await.unwrap();
    assert_eq!(json["status"], "healthy");
    assert!(json["timestamp"].is_number());
}

#[tokio::test]
async fn test_metrics_endpoint() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .uri("/metrics")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = read_body(response.into_body()).await.unwrap();
    assert!(json["uptime_seconds"].is_number());
    assert!(json["memory_usage_mb"].is_number());
    assert!(json["cpu_usage_percent"].is_number());
    assert!(json["total_requests"].is_number());
    assert!(json["requests_per_minute"].is_number());
}

#[tokio::test]
async fn test_create_short_url() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/shorturl")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "original_url": "https://www.rust-lang.org"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = read_body(response.into_body()).await.unwrap();
    assert_eq!(json["original_url"], "https://www.rust-lang.org");
    assert!(json["short_url"].is_string());
}

#[tokio::test]
async fn test_invalid_url() {
    let app = setup_test_app().await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/shorturl")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "original_url": "not-a-valid-url"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_get_all_urls() {
    let app = setup_test_app().await;

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/shorturl")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "original_url": "https://www.rust-lang.org"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::OK);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/shorturls")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = read_body(response.into_body()).await.unwrap();
    assert!(json.as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn test_redirect() {
    let app = setup_test_app().await;

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/shorturl")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    json!({
                        "original_url": "https://www.rust-lang.org"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    let json = read_body(create_response.into_body()).await.unwrap();
    let short_url = json["short_url"].as_str().unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/shorturl/{}", short_url))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::PERMANENT_REDIRECT);
    assert_eq!(response.headers()["location"], "https://www.rust-lang.org");
}
