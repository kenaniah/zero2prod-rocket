use crate::fairings::request_id::RequestId;
use rocket::http::Status;

#[get("/")]
pub fn health_check(rid: RequestId) -> Status {
    println!("   >> Request ID: {}", *rid);
    Status::Ok
}
