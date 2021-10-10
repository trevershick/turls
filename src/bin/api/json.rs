use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShortenedCreate {
    pub keyword: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
}

