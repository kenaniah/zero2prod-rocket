use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use uuid::Uuid;

pub struct RequestId(Option<String>);

#[derive(Default)]
pub struct RequestIdentity {
    read_from_header: bool,
    output_as_header: bool,
}

impl RequestIdentity {
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
impl Fairing for RequestIdentity {
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
            RequestId(Some(
                identifier.unwrap_or_else(|| Uuid::new_v4().hyphenated().to_string()),
            ))
        });
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if self.output_as_header {
            if let RequestId(Some(id)) = request.local_cache(|| RequestId(None)) {
                response.set_raw_header("X-Request-Id", id);
            }
        }
    }
}
