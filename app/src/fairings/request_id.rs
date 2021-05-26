use rocket::{Rocket, Request, Data, Response, Build, Orbit};
use rocket::fairing::{self, Fairing, Info, Kind};

#[derive(Default)]
pub struct RequestId {
    output_header: bool
}

#[rocket::async_trait]
impl Fairing for RequestId {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response | Kind::Singleton
        }
    }
}
