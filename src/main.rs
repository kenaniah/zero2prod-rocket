#[macro_use]
extern crate rocket;
extern crate zero2prod;

use rocket::Rocket;

mod routes;

#[launch]
pub fn ignite() -> Rocket {
    zero2prod::app()
}