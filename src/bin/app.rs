#[macro_use]
extern crate rocket;

#[cfg(test)]
extern crate uuid;

#[cfg_attr(test, macro_use)]
extern crate serde_json;

extern crate bincode;
extern crate sled;
extern crate yansi;

extern crate metrics;
extern crate metrics_exporter_prometheus;

#[macro_use]
mod testing;

mod api;

use lib_turls::{db, Error};
use metrics::{histogram, increment_counter};
use rocket::fairing;
use rocket::http::Status;
use rocket::log::PaintExt;
use rocket::response::Redirect;
use rocket::{Build, Rocket, State};
use yansi::Paint;

#[derive(Clone, Debug, Default)]
pub struct Version {
    pub full: String,
}

#[get("/")]
fn index(version: &State<Version>) -> String {
    format!("Turls v{}", version.full)
}

#[get("/<keyword>", format = "html")]
fn goto_keyword(db: &State<db::Db>, keyword: &str) -> Result<Redirect, Status> {
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

#[derive(Debug)]
pub struct TurlsVersionFairing;

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
        let config = db.config();

        info!("{}{}:", Paint::emoji("🐸 "), Paint::magenta("TurlsDb"));
        info_!("{}: {}", "path", config.path);
        info_!("{}: {}", "temporary", config.temporary);
        Ok(rocket.manage(db))
    }
}

#[rocket::async_trait]
impl fairing::Fairing for TurlsVersionFairing {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "TurlsVersion",
            kind: fairing::Kind::Ignite | fairing::Kind::Response,
        }
    }

    async fn on_response<'r>(
        &self,
        _req: &'r rocket::Request<'_>,
        _res: &mut rocket::Response<'r>,
    ) {
        _req.rocket()
            .state::<Version>()
            .map(|v| _res.set_raw_header("X-Turls-Version", v.full.clone()));
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        info!("{}{}:", Paint::emoji("🐸 "), Paint::magenta("TurlsVersion"));
        match rocket.state::<Version>() {
            Some(v) => {
                info_!("{}: {}", "full", v.full);
            }
            _ => {}
        };
        Ok(rocket)
    }
}

struct TurlsMetricsFairing;
#[derive(Copy, Clone)]
struct TimerStart(Option<std::time::SystemTime>);
#[rocket::async_trait]
impl fairing::Fairing for TurlsMetricsFairing {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "TurlsMetrics",
            kind: fairing::Kind::Request | fairing::Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut rocket::Request<'_>, _data: &mut rocket::Data<'_>) {
        req.local_cache(|| TimerStart(Some(std::time::SystemTime::now())));
    }

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        let start_time = req.local_cache(|| TimerStart(None));
        if let Some(Ok(duration)) = start_time.0.map(|st| st.elapsed()) {
            if let Some(route) = req.route() {
                if let Some(nm) = &route.name {
                    let labels = [
                        ("route", nm.to_string()),
                        ("status", format!("{}", res.status().code)),
                    ];
                    increment_counter!("route_hit_count", &labels);
                    histogram!("route_response_time", duration, &labels);
                }
            }
            let ms = duration.as_secs() * 1000 + duration.subsec_millis() as u64;
            res.set_raw_header("X-Response-Time", format!("{} ms", ms));
        }
    }
}

#[launch]
pub fn rocket() -> _ {
    let version = Version {
        full: env!("CARGO_PKG_VERSION").to_string(),
    };
    use metrics_exporter_prometheus::PrometheusBuilder;

    PrometheusBuilder::new().install().expect("prometheus");

    rocket::build()
        .manage(version)
        .attach(TurlsDbFairing {})
        .attach(TurlsMetricsFairing {})
        .attach(TurlsVersionFairing {})
        .mount("/", routes![index])
        .mount("/", routes![goto_keyword])
        .mount("/api/v1", api::routes())
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use super::*;
    use rocket::local::blocking::Client;

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
        assert_eq!(
            "https://u1",
            response.headers().get_one("Location").unwrap()
        );
    }
}
