mod error;
mod handlers;
mod models;
mod services;
use crate::handlers::url_handler::UrlHandler;
use crate::handlers::{health_routes, init_metrics};
use crate::services::url_service::UrlService;

use axum::{
    routing::{get, post},
    Router,
};
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv::dotenv().ok();

    // Get MongoDB URI from environment variable
    let db_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    println!("🔍 Connecting to MongoDB at: {}", db_uri);

    let client_options = ClientOptions::parse(&db_uri).await?;
    let client = Client::with_options(client_options)?;

    // Verify the connection
    match client
        .database("admin")
        .run_command(mongodb::bson::doc! {"ping": 1})
        .await
    {
        Ok(_) => println!("✅ Connected to MongoDB successfully!"),
        Err(e) => {
            println!("❌ Failed to connect to MongoDB: {}", e);
            return Err(e);
        }
    }

    let db = client.database("url_shortener");
    let collection = db.collection("urls");

    let url_service = UrlService::new(collection);
    let url_handler = Arc::new(UrlHandler::new(url_service));

    init_metrics();

    let create_handler = Arc::clone(&url_handler);
    let redirect_handler = Arc::clone(&url_handler);
    let get_all_handler = Arc::clone(&url_handler);

    let app = Router::new()
        .route("/", get(|| async { "✅ URL Shortener is running!" }))
        .merge(health_routes())
        .route(
            "/api/shorturl",
            post(move |payload| async move { create_handler.create_short_url(payload).await }),
        )
        .route(
            "/api/shorturl/{short_url}",
            get(move |path| async move { redirect_handler.redirect_to_original(path).await }),
        )
        .route(
            "/api/shorturls",
            get(move || async move { get_all_handler.get_all_short_urls().await }),
        );

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 URL Shortener running on http://0.0.0.0:{}", port);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
