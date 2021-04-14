#[macro_use]
extern crate rocket;

mod routes;

pub fn app() -> rocket::Rocket {
    rocket::ignite().mount("/health_check", routes![routes::health_check::health_check])
}
