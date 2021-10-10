use super::validation;
use serde_json::{json,Value};

#[allow(dead_code)]
#[derive(Responder)]
pub enum JsonApiError {
    #[response(status = 400, content_type = "json")]
    BadRequest(Value),

    #[response(status = 409, content_type = "json")]
    Conflict(Value),

    #[response(status = 404, content_type = "json")]
    NotFound(Value),

    #[response(status = 500, content_type = "json")]
    GenericError(Value),
}

impl From<&validation::Error> for Value {
    fn from(e: &validation::Error) -> Value {
        use super::validation::Error::*;
        match e {
            ValueIsRequired(f) => json!({
                "field": f,
                "rule": "required",
            }),
            ValueTooShort(f, min, actual, sz) => json!({
                "field": f,
                "rule": "tooshort",
                "min": min,
                "value": actual,
                "len": sz,
            }),
            ValueTooLong(f, max, actual, sz) => json!({
                "field": f,
                "rule": "toolong",
                "max": max,
                "value": actual,
                "len": sz,
            }),
            InvalidShortenedCreate(s_create, errs) => {
                let j = json!({ "object": s_create ,
                    "errors": errs.iter().map(|o|Value::from(o)).collect::<Vec<Value>>(),
                });
                return j;
            }
        }
    }
}

impl From<validation::Error> for JsonApiError {
    fn from(e: validation::Error) -> JsonApiError {
        use JsonApiError::*;
        BadRequest(Value::from(&e))
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
