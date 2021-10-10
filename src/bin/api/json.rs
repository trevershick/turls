use lib_turls::model;
use rocket::form::FromForm;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SearchResults {
    pub params: SearchParams,
    pub start: usize,
    pub end: usize,
    pub urls: Vec<model::Shortened>,
}

#[derive(FromForm, Debug, Deserialize, Serialize, Clone, Default)]
pub struct SearchParams {
    pub start: Option<usize>,
    pub page_size: Option<usize>,
    // genereric term search
    pub term: Option<String>,
}

impl From<SearchParams> for model::SearchParams {
    fn from(s: SearchParams) -> Self {
        Self {
            term: s.term.unwrap_or("".to_owned()),
            page_size: s.page_size.unwrap_or(25),
            start: s.start.unwrap_or(0),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShortenedCreate {
    pub keyword: Option<String>,
    pub url: Option<String>,
    pub title: Option<String>,
}
