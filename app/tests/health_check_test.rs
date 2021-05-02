mod setup;

#[cfg(test)]
mod test {
    use rocket::http::Status;

    #[test]
    fn health_check() {
        let mut env = super::setup::MockEnvironment::new();
        let client = env.client();
        let response = client.get("/health_check").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), None);
    }
}
