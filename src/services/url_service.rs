use crate::error::AppError;
use base62::encode;
use futures_util::TryStreamExt;
use mongodb::{bson::doc, Collection};
use rand::Rng;
use serde::{Deserialize, Serialize};
use url::Url as UrlParser;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortUrl {
    pub short_url: String,
    pub original_url: String,
}

pub struct UrlService {
    collection: Collection<ShortUrl>,
}

impl UrlService {
    pub fn new(collection: Collection<ShortUrl>) -> Self {
        Self { collection }
    }

    pub async fn create_short_url(&self, original_url: String) -> Result<ShortUrl, AppError> {
        // Validate URL
        if !Self::is_valid_url(&original_url) {
            return Err(AppError::InvalidUrl("Invalid URL format".to_string()));
        }

        // Generate short URL
        let short_url = self.generate_short_url();

        let url = ShortUrl {
            original_url,
            short_url,
        };

        self.collection
            .insert_one(&url)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(url)
    }

    pub async fn get_original_url(&self, short_url: String) -> Result<String, AppError> {
        let url = self
            .collection
            .find_one(doc! { "short_url": &short_url })
            .await
            .map_err(AppError::DatabaseError)?
            .ok_or_else(|| AppError::NotFound("URL not found".to_string()))?;

        Ok(url.original_url)
    }

    fn generate_short_url(&self) -> String {
        let random_number: u32 = rand::rng().random();
        encode(random_number as u64)
    }

    fn is_valid_url(url: &str) -> bool {
        match UrlParser::parse(url) {
            Ok(parsed) => parsed.scheme() == "http" || parsed.scheme() == "https",
            Err(_) => false,
        }
    }

    pub async fn get_all_short_urls(&self) -> Result<Vec<ShortUrl>, AppError> {
        let mut cursor = self
            .collection
            .find(doc! {})
            .await
            .map_err(AppError::DatabaseError)?;

        let mut results = Vec::new();
        while let Some(doc_result) = cursor.try_next().await.map_err(AppError::DatabaseError)? {
            results.push(doc_result);
        }

        Ok(results)
    }
}
