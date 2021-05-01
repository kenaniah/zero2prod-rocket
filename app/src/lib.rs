#[macro_use]
extern crate rocket;
#[macro_use]
extern crate db;

mod routes;

use std::ops::Deref;

use rocket::{request::{Request, FromRequest}};
use routes::{health_check, subscriptions};

use db::{PgConnection, r2d2::{ConnectionManager, Pool, PooledConnection}};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub struct MainDatabase(PgPool);
pub struct MainConnection(PgPooledConnection);

impl Deref for MainConnection {
    type Target = PgPooledConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MainDatabase {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        request.guard::<MainDatabase>().await
    }

}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MainConnection {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        let res = request.guard::<MainDatabase>().await;
        if let Some(pool) = res.succeeded() {
            rocket::outcome::Outcome::Success(MainConnection(pool.0.get().unwrap()))
        } else {
            rocket::request::Outcome::Failure((rocket::http::Status::ServiceUnavailable, ()))
        }
    }

}

pub fn app() -> rocket::Rocket<rocket::Build> {
    let manager: ConnectionManager<PgConnection> = ConnectionManager::new("postgres:///zero2prod");
    let pool = Pool::builder().build(manager).unwrap();

    rocket::build()
        .manage::<MainDatabase>(MainDatabase(pool))
        .mount("/health_check", routes![health_check::health_check])
        .mount("/subscriptions", routes![subscriptions::subscribe])
}
