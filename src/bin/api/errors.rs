use serde_json::json;
use super::json;

#[allow(dead_code)]
#[derive(Responder)]
pub enum JsonApiError {
    #[response(status = 400, content_type = "json")]
    BadRequest(serde_json::Value),

    #[response(status = 409, content_type = "json")]
    Conflict(serde_json::Value),

    #[response(status = 404, content_type = "json")]
    NotFound(serde_json::Value),

    #[response(status = 500, content_type = "json")]
    GenericError(serde_json::Value),
}

impl From<json::Error> for JsonApiError {
    fn from(e: json::Error) -> JsonApiError {
        use super::json::Error::*;
        use JsonApiError::*;
        match e {
            InvalidShortenedCreate(s,e) => BadRequest(json!({
                "object": s,
                "errors": e,
            }))
        }
    }
}

impl From<lib_turls::Error> for JsonApiError {
    fn from(e: lib_turls::Error) -> JsonApiError {
        use lib_turls::Error::*;
        use JsonApiError::*;
        match e {
            DbError(x) => GenericError(json!({ "message": format!("{}", x) })),
            UrlDoesNotExist(id) => NotFound(json!({ 
                "id": id,
                "message": "keyword does not exist",
            })),
            KeywordDoesNotExist(kw) => NotFound(json!({ 
                "keyword": kw,
                "message": "keyword does not exist",
            })),
            KeywordAlreadyExists(kw) => Conflict(json!({ 
                "keyword": kw,
                "message": "keyword already exists",
            })),
        }
    }
}

