#[macro_use]
extern crate rocket;
extern crate zero2prod;

mod routes;

#[launch]
pub fn ignite() -> rocket::Rocket {
    zero2prod::app()
}
