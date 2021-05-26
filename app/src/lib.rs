#![allow(clippy::unit_arg)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate db;

mod routes;
mod fairings;

use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use routes::{health_check, subscriptions};

use fairings::request_id::RequestId;

use db::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub struct MainDatabase(PgPool);
pub struct MainConnection(PgPooledConnection);

impl Deref for MainDatabase {
    type Target = PgPool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for MainConnection {
    type Target = PgPooledConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MainConnection {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let res = request.rocket().state::<MainDatabase>();
        if let Some(pool) = res {
            match pool.get() {
                Ok(conn) => Outcome::Success(MainConnection(conn)),
                Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
            }
        } else {
            Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

pub fn app(db_url: &str) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(RequestId::default())
        .manage::<MainDatabase>(main_database_pool(db_url))
        .mount("/health_check", routes![health_check::health_check])
        .mount("/subscriptions", routes![subscriptions::subscribe])
}

/// Builds a connection pool for the main database
fn main_database_pool(db_url: &str) -> MainDatabase {
    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(db_url);
    let pool = Pool::builder()
        .connection_timeout(std::time::Duration::from_millis(5000))
        .build(manager)
        .unwrap();
    MainDatabase(pool)
}
