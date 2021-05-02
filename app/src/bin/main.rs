#[macro_use]
extern crate rocket;
extern crate app;

#[launch]
pub fn ignite() -> rocket::Rocket<rocket::Build> {
    app::app("postgres:///zero2prod")
}
