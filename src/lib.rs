#[macro_use]
extern crate rocket;

use rocket::Rocket;

mod routes;

pub fn app() -> Rocket {
    rocket::ignite().mount("/health_check", routes![routes::health_check::health_check])
}
