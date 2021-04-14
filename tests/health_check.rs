#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use zero2prod::app;

    #[test]
    fn health_check() {
        let rocket = app();
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/health_check").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), None);
    }
}
