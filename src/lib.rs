#[macro_use]
extern crate rocket;
use rocket::Rocket;

pub mod configuration;
mod routes;
use routes::{health_check, subscriptions};

pub fn app() -> Rocket {
    rocket::ignite()
        .mount("/health_check", routes![health_check::health_check])
        .mount("/subscriptions", routes![subscriptions::subscribe])
}
