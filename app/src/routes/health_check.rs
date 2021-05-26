use rocket::http::Status;
use crate::fairings::request_id::RequestId;

#[get("/")]
pub fn health_check(rid: RequestId) -> Status {
    println!("   >> Request ID: {}", *rid);
    Status::Ok
}
