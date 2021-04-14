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

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn health_check() {
        let rocket = super::rocket();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/health_check").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), None);
    }

}
