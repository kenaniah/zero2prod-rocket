#[macro_use]
extern crate rocket;
extern crate app;

use std::env;

#[launch]
pub fn ignite() -> rocket::Rocket<rocket::Build> {
    app::app(&env::var("DATABASE_URL").unwrap_or_else(|_| "postgres:///zero2prod".into()))
}
