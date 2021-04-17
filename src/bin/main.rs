#[macro_use]
extern crate rocket;
extern crate zero2prod;

#[launch]
pub fn ignite() -> rocket::Rocket {
    zero2prod::app()
}
