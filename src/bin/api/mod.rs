mod errors;
mod json;
mod validation;

use errors::JsonApiError;
use lib_turls::db;
use lib_turls::model;
use rocket::serde::json::Json;
use validation::Validated;

pub fn routes() -> Vec<rocket::Route> {
    let mut x: Vec<rocket::Route> = routes![]; //api::index];
    x.extend(routes![get_shortened]);
    x.extend(routes![shorten]);
    x.extend(routes![search]);
    x.extend(routes![expand_keyword]);
    return x;
}

#[post("/urls", format = "application/json", data = "<url>")]
async fn shorten(
    db: &rocket::State<db::Db>,
    url: Json<json::ShortenedCreate>,
) -> Result<Json<model::Shortened>, JsonApiError> {
    url.validate()?;

    db.insert_url(
        url.keyword.as_deref().unwrap(),
        url.url.as_deref().unwrap(),
        url.title.as_deref(),
    )
    .map(|s| Json(s))
    .map_err(|e| JsonApiError::from(e))
}

#[get("/keywords/<keyword>", format = "application/json")]
async fn expand_keyword(
    db: &rocket::State<db::Db>,
    keyword: String,
) -> Result<Json<String>, JsonApiError> {
    let url = db.find_url_by_keyword(&keyword)?;
    Ok(Json(url))
}

#[get("/urls/<id>", format = "application/json")]
async fn get_shortened(
    db: &rocket::State<db::Db>,
    id: u64,
) -> Result<Json<model::Shortened>, JsonApiError> {
    let url = db.find_url(id)?;
    Ok(Json(url))
}

#[get("/search?<p..>", format = "application/json")]
fn search(
    db: &rocket::State<db::Db>,
    p: json::SearchParams,
) -> Result<Json<json::SearchResults>, JsonApiError> {
    info!("Search {:?}", p);
    let sp : model::SearchParams = p.clone().into();
    let iter : Vec<model::Shortened> = db.search(&sp)?.collect();
    Ok(Json(json::SearchResults {
        start: sp.start,
        end: sp.start + iter.len(),
        urls: iter,
        params: p,
    }))
}

#[cfg(test)]
mod tests {
    use lib_turls::db::{Config, Db};
    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    use serde_json::Value;
    use std::env;

    fn client(_n: &str) -> Client {
        let s: String = env::var("CARGO_TARGET_DIR")
            .map(|o| o.to_string())
            .ok()
            .unwrap_or(".".to_owned());
        let mut p = std::path::PathBuf::from(s);
        p.push(uuid::Uuid::new_v4().to_string());
        let path = p.to_string_lossy().to_owned().to_string();
        debug!("Writing db files into {}", path);

        let config = Config {
            path,
            temporary: true,
        };
        let db = Db::init(&config).expect("init");
        let rocket = rocket::build().manage(db).mount("/", super::routes());
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
        assert_eq!(json["message"], "keyword does not exist");
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
