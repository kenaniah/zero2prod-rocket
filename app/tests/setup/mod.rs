use rocket::local::blocking::Client;

pub fn blocking_client() -> Client {
    let rocket = app::app();
    Client::tracked(rocket).expect("valid rocket instance")
}
