use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShortenedCreate {
    pub keyword: String,
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ValidationError {
    pub field: String,
    pub rule: String,
}

pub enum Error<E> where E : Deserialize {
    InvalidShortenedCreate(ShortenedCreate, Vec<Box<E>>) 
}

fn check_size(value: &str, field: &str, min: usize, max: usize) -> Option<ValidationError> {
    if value.len() < min {
        return Some(ValidationError{
            field: field.to_owned(),
            rule: "too_short".to_owned(),
        });
    }
    if value.len() > max {
        return Some(ValidationError{
            field: field.to_owned(),
            rule: "too_long".to_owned(),
        });
    }
    None
}

impl ShortenedCreate {
    pub fn validate(self: &ShortenedCreate) -> Result<(), Error>  {
        let mut errors = Vec::<ValidationError>::new();
        check_size(&self.url, "url", 2, 255).map(|it| errors.push(it));
        check_size(&self.keyword, "keyword", 2, 25).map(|it| errors.push(it));
        match errors.len() {
            0 => Ok(()),
            _ => Err(Error::InvalidShortenedCreate(self.clone(), errors)),
        }
    }
}
