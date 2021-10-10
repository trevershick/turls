use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ShortenedCreate {
    pub keyword: String,
    pub url: String,
    pub title: Option<String>,
    /* created */
}
