use lib_turls::{db, model::Shortened};
use rocket::get;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct ShortenedCreate {
    pub keyword: String,
    pub url: String,
    pub title: Option<String>,
    /* created */
}

#[allow(dead_code)]
#[derive(Responder)]
pub enum JsonApiError {
    #[response(status = 404, content_type = "json")]
    NotFound(serde_json::Value),
    #[response(status = 500, content_type = "json")]
    GenericError(serde_json::Value),
}

impl From<lib_turls::Error> for JsonApiError {
    fn from(e: lib_turls::Error) -> JsonApiError {
        JsonApiError::GenericError(json!({ "message": format!("{}", e) }))
    }
}
fn not_found(id: u64) -> JsonApiError {
    JsonApiError::NotFound(json!({ "id": id, "message": "not found" }))
}

#[post("/urls", format = "application/json", data = "<url>")]
pub fn shorten(
    db: &rocket::State<db::Db>,
    url: Json<ShortenedCreate>,
) -> Result<Json<Shortened>, JsonApiError> {
    db.insert_url(&url.keyword, &url.url, url.title.as_deref())
        .map(|s| Json(s))
        .map_err(|e| JsonApiError::from(e))
}

#[get("/urls/<id>", format = "application/json")]
pub fn get_shortened(db: &rocket::State<db::Db>, id: u64) -> Result<Json<Shortened>, JsonApiError> {
    db.find_url(id)
        .map_err(|e| JsonApiError::from(e))?
        .map(Json)
        .ok_or(not_found(id))
}

#[cfg(test)]
mod tests {
    use lib_turls::db::{Config, Db};
    use rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use serde_json::Value;

    fn client(_n: &str) -> Client {
        let config = Config {
            path: uuid::Uuid::new_v4().to_string(),
            temporary: true,
        };
        let db = Db::init(&config).expect("init");
        let rocket = rocket::build().manage(db).mount("/", crate::api::routes());
        Client::tracked(rocket).unwrap()
    }

    #[test]
    fn get_shortened_not_found() {
        let client = client("get_shortened_not_found");
        let response = client.get("/urls/0").header(ContentType::JSON).dispatch();
        assert_eq!(response.status(), Status::NotFound);
        assert_json!(response);
        let s = response.into_string().unwrap();
        let json: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(json["id"], 0);
        assert_eq!(json["message"], "not found");
    }

    #[test]
    fn get_shortened_1() {
        let client = client("get_shortened_1");
        let pr = client
            .post("/urls")
            .header(ContentType::JSON)
            .body(json!({ "keyword": "k1", "url": "u1" }).to_string())
            .dispatch();
        assert_ok!(pr);

        let expected = json!({ 
            "id": 0,
            "title": Option::<String>::None, 
            "keyword": "k1", 
            "url": "u1" });
        let response = client.get("/urls/0").header(ContentType::JSON).dispatch();
        assert_ok!(response);
        assert_json!(response);
        assert_json_eq!(response, expected);
    }
}
