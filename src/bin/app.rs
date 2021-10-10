#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

#[macro_use] extern crate log;

extern crate yansi;
extern crate bincode;
extern crate sled;

#[cfg(test)]
extern crate uuid;

#[macro_use]
mod testing;

mod api;

use rocket::log::PaintExt;
use yansi::Paint;

use rocket::{Rocket,Build,State};
use rocket::fairing;
use lib_turls::{db, Error};
use rocket::response::Redirect;
use rocket::http::Status;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<keyword>",format="html")]
fn goto_keyword(db: &State<db::Db>, keyword: &str) -> Result<Redirect,Status> {
    //Redirect::to(uri!("https://rocket.rs/bye", hello(name, age), "?bye#now"))
    use Error::*;
    let url = db.find_url_by_keyword(keyword);
    match url {
        Ok(u) => Ok(Redirect::temporary(u.to_owned())),
        Err(UrlDoesNotExist(_)) => Err(Status::NotFound),
        Err(_) => Err(Status::ServiceUnavailable),
    }
}

#[derive(Debug)]
pub struct TurlsDbFairing;

#[rocket::async_trait]
impl fairing::Fairing for TurlsDbFairing {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "TurlsDB",
            kind: fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let config = db::Config {
            ..Default::default()
        };
        let db = db::Db::init(&config).unwrap();

        info!("{}{}:", Paint::emoji("🐸 "), Paint::magenta("TurlsDb"));
        info_!("{}: {}", "path", db.config().path);
        info_!("{}: {}", "temporary", db.config().temporary);
        Ok(rocket.manage(db))
    }
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(TurlsDbFairing {})
        .mount("/", routes![index])
        .mount("/", routes![goto_keyword])
        .mount("/api/v1", api::routes())
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::local::blocking::Client;
    use super::*;

    fn client(_n: &str) -> Client {
        let config = db::Config {
            path: uuid::Uuid::new_v4().to_string(),
            temporary: true,
        };
        let db = db::Db::init(&config).expect("init");
        let rocket = rocket::build()
            .manage(db)
        .mount("/", routes![goto_keyword])
        .mount("/api/v1", api::routes());
        Client::tracked(rocket).unwrap()
    }
    #[test]
    fn index() {
        let client = client("");
        let pr = client
            .post("/api/v1/urls")
            .header(rocket::http::ContentType::JSON)
            .body(json!({ "keyword": "k1", "url": "https://u1" }).to_string())
            .dispatch();
        assert_ok!(pr);
        let response = client.get("/k1").dispatch();
        assert_eq!(rocket::http::Status::TemporaryRedirect, response.status());
        assert_eq!("https://u1",response.headers().get_one("Location").unwrap());
    }
}
