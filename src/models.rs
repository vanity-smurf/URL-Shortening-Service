use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewURL {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenedURL {
    pub id: u32,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub clicks: u32,
}


