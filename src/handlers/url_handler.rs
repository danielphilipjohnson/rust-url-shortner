use crate::{error::AppError, services::url_service::UrlService};
use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUrlRequest {
    pub original_url: String,
}

pub struct UrlHandler {
    url_service: UrlService,
}

impl UrlHandler {
    pub fn new(url_service: UrlService) -> Self {
        Self { url_service }
    }

    pub async fn create_short_url(
        &self,
        Json(payload): Json<CreateUrlRequest>,
    ) -> Result<impl IntoResponse, AppError> {
        let url = self
            .url_service
            .create_short_url(payload.original_url)
            .await?;
        Ok(Json(url))
    }

    pub async fn redirect_to_original(
        &self,
        Path(short_url): Path<String>,
    ) -> Result<impl IntoResponse, AppError> {
        let original_url = self.url_service.get_original_url(short_url).await?;
        Ok(Redirect::permanent(&original_url.as_str()))
    }

    pub async fn get_all_short_urls(&self) -> impl IntoResponse {
        match self.url_service.get_all_short_urls().await {
            Ok(urls) => Json(urls).into_response(),
            Err(e) => {
                eprintln!("❌ Error fetching URLs: {:?}", e);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to fetch URLs",
                )
                    .into_response()
            }
        }
    }
}
