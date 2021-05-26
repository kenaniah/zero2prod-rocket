use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request;
use rocket::request::FromRequest;
use rocket::{Data, Request, Response};
use uuid::Uuid;

struct Content(Option<String>);

#[derive(Default)]
pub struct IdentityFairing {
    read_from_header: bool,
    output_as_header: bool,
}

impl IdentityFairing {
    #[allow(dead_code)]
    pub fn set_read_from_header(mut self, val: bool) -> Self {
        self.read_from_header = val;
        self
    }
    #[allow(dead_code)]
    pub fn set_output_as_header(mut self, val: bool) -> Self {
        self.output_as_header = val;
        self
    }
}

#[rocket::async_trait]
impl Fairing for IdentityFairing {
    fn info(&self) -> Info {
        Info {
            name: "RequestId",
            kind: Kind::Request | Kind::Response | Kind::Singleton,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data) {
        let identifier = match self.read_from_header {
            true => request
                .headers()
                .get_one("X-Request-ID")
                .map(|x| x.to_string()),
            false => None,
        };

        request.local_cache(|| {
            Content(Some(
                identifier.unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string()),
            ))
        });
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if self.output_as_header {
            if let Content(Some(id)) = request.local_cache(|| Content(None)) {
                response.set_raw_header("X-Request-Id", id);
            }
        }
    }
}

/// Request guard used to retrieve the identifier of a request.
#[derive(Clone, Copy)]
pub struct RequestId<'r>(pub &'r str);

use std::ops::Deref;
impl<'r> Deref for RequestId<'r> {
    type Target = &'r str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Allows a route to access the request's identifier.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestId<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, ()> {
        match &*request.local_cache(|| Content(None)) {
            Content(Some(id)) => request::Outcome::Success(RequestId(&id)),
            Content(None) => request::Outcome::Failure((Status::InternalServerError, ())),
        }
    }
}
