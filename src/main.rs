#[macro_use]
extern crate rocket;

use rocket::http::Status;

#[get("/health_check")]
fn health_check() -> Status {
    Status::Ok
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![health_check])
}
