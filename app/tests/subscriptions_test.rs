mod setup;

use db;
use rocket::http::uri::fmt::{Query, UriDisplay};
use rocket::UriDisplayQuery;

#[derive(UriDisplayQuery)]
struct Body<'a> {
    name: Option<&'a str>,
    email: Option<&'a str>,
}

impl Body<'_> {
    /// Returns the struct as an URI-formatted string
    pub fn to_uri(&self) -> String {
        format!("{}", &self as &dyn UriDisplay<Query>)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use db::models::Subscription;
    use diesel::prelude::*;
    use rocket::http::{ContentType, Status};

    #[test]
    fn request_body() {
        let body = Body {
            name: Some("Some One"),
            email: Some("foo@example.com"),
        };
        assert_eq!(body.to_uri(), "name=Some%20One&email=foo@example.com");
        let body = Body {
            name: None,
            email: None,
        };
        assert_eq!(body.to_uri(), "");
    }

    #[test]
    fn subscribe_returns_a_200_for_valid_form_data() {
        // Set up the mock environment
        let mut env = super::setup::MockEnvironment::new();

        // Submit the POST request
        let client = env.client();
        let response = client
            .post("/subscriptions")
            .header(ContentType::Form)
            .body(
                Body {
                    name: "Some One".into(),
                    email: "someone@gmail.com".into(),
                }
                .to_uri(),
            )
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), None);

        // Check the database state
        let conn = env.connection();
        use db::schema::subscriptions::dsl::*;
        let row: Subscription = subscriptions.order_by(id.desc()).first(&conn).unwrap();
        assert_eq!(row.name, "Some One");
        assert_eq!(row.email, "someone@gmail.com");
    }

    #[test]
    fn subscribe_returns_a_422_when_data_is_missing() {
        let mut env = super::setup::MockEnvironment::new();
        let client = env.client();
        let test_cases = vec![
            ("name=some%20one", "missing the email"),
            ("email=someone@gmail.com", "missing the name"),
            ("", "missing both name and email"),
        ];

        for (invalid_body, error_message) in test_cases {
            let response = client
                .post("/subscriptions")
                .header(ContentType::Form)
                .body(invalid_body)
                .dispatch();
            assert_eq!(
                response.status(),
                Status::UnprocessableEntity,
                "The API did not fail with HTTP 422 when the payload was {}.",
                error_message
            );
        }
    }
}
