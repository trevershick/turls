use lib_turls::model;
use log::{debug, info, warn};
use rocket::get;
use rocket::serde::json::Json;
use model::Shortened;
use serde::{Deserialize, Serialize};
use zerocopy::AsBytes;

const TREE_URLS : &str = "urls";

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
    Error(serde_json::Value),
    #[response(status = 500, content_type = "json")]
    SimpleError(&'static str),
}

#[post("/urls", format = "application/json", data = "<url>")]
pub fn shorten(
    db: &rocket::State<sled::Db>,
    url: Json<ShortenedCreate>,
) -> Result<Json<Shortened>, JsonApiError> {
    let new_shortened = Shortened {
        id: db.generate_id().unwrap(),
        keyword: url.keyword.clone(),
        title: url.title.clone(),
        url: url.url.clone(),
    };
    debug!("New Shortened Url: {:?}", new_shortened);
    let dbid = Shortened::db_id(new_shortened.id);
    let data = new_shortened.to_ivec();
    debug!("Data size to insert is {}", data.iter().len());
    db.open_tree(TREE_URLS)
        .expect("should open db")
        .insert(dbid.as_bytes(), data)
        .expect("Should have inserted");
    Ok(Json(new_shortened))
}

#[get("/urls/<id>", format = "application/json")]
pub fn get_shortened(
    db: &rocket::State<sled::Db>,
    id: u64,
) -> Result<Json<Shortened>, JsonApiError> {
    let dbid = Shortened::db_id(id);
    debug!("Searching for {}", dbid);
    let record = match db.open_tree(TREE_URLS)
        .expect("db should open")
        .get(dbid.as_bytes()) {
            Ok(r) => r,
            Err(e) => {
                return Result::Err( JsonApiError::Error(json!({ "error": e.to_string() })));
            }
        };

    match record {
        Some(ivec) => Ok(Json(Shortened::from(ivec))),
        None => Result::Err(JsonApiError::NotFound(
            json!({ "id": id, "message": "not found"}),
        )),
    }
}

#[cfg(test)]
mod tests {
    use rocket;
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use serde_json::Value;

    fn client(n: &str) -> Client {
        let sleddb = sled::Config::new().path(n).temporary(true).open().unwrap();
        let rocket = rocket::build()
            .mount("/", crate::api::routes())
            .manage(sleddb);
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
            .body(
                json!({
                    "keyword": "k1",
                    "url": "u1",
                })
                .to_string(),
            )
            .dispatch();
        assert_ok!(pr);

        let response = client.get("/urls/0").header(ContentType::JSON).dispatch();
        assert_ok!(response);
        assert_json!(response);
        //let s = response.into_string().unwrap();
        //let json: Value = serde_json::from_str(&s).unwrap();
        //assert_eq!(json["id"], 1);
        //assert_eq!(json["title"], "t");
        //assert_eq!(json["url"], "https://google.com");
        //assert_eq!(json["keyword"], "key");
    }
}
