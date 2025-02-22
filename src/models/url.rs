use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Url {
    pub original_url: String,
    pub short_url: String,
    pub created_at: DateTime,
}
