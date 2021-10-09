#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate log;

extern crate sled;
extern crate bincode;

#[macro_use]
mod testing;

mod api;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[derive(Debug,Clone)]
pub struct SledDbFairing;

#[derive(Debug)]
pub struct SledDbError;

#[rocket::async_trait]
impl rocket::fairing::Fairing for SledDbFairing {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "SledDB Fairing",
            kind: rocket::fairing::Kind::Ignite,
        }
    }
    async fn on_ignite(&self, rocket: rocket::Rocket<rocket::Build>) -> rocket::fairing::Result {
        println!("Igniting and setting up data @ turlsdata");
        let s = sled::Config::new()
            .path("turlsdata")
            .open().unwrap();
            //let s = sled::open("turlsdata").unwrap();
            Ok(rocket.manage(s))
    }
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(SledDbFairing{})
        .mount("/", routes![index])
        .mount("/api/v1", api::routes())
}

#[cfg(test)]
mod tests {
    use super::rocket;
    use rocket::local::blocking::Client;

    //#[test]
    //fn index() {
    //    let rocket = rocket::build().mount("/", routes![super::index]);
    //    let client = Client::tracked(rocket).unwrap();
    //    let response = client.get("/").dispatch();
    //    assert_ok!(response);
    //    assert_eq!(response.into_string().unwrap(), "Hello, world!");
    //}
}
