// reexport everything in api.rs
mod api;
pub use api::*;

pub fn routes() -> Vec<rocket::Route> {
    let mut x: Vec<rocket::Route> = routes![]; //api::index];
    x.extend(routes![api::get_shortened]);
    x.extend(routes![api::shorten]);
    return x;
}
