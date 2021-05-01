use rocket::local::blocking::Client;

pub fn blocking_client() -> Client {
    let rocket = zero2prod::app();
    Client::tracked(rocket).expect("valid rocket instance")
}
