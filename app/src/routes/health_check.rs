use rocket::http::Status;

#[get("/")]
pub fn health_check() -> Status {
    Status::Ok
}
