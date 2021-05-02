#[macro_use]
extern crate rocket;
#[macro_use]
extern crate db;

mod routes;

use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use routes::{health_check, subscriptions};

use db::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
struct MainDatabase(PgPool);
pub struct MainConnection(PgPooledConnection);

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
            match pool.0.get() {
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
        .manage::<MainDatabase>(main_database_pool(db_url))
        .mount("/health_check", routes![health_check::health_check])
        .mount("/subscriptions", routes![subscriptions::subscribe])
}

/// Builds a connection pool for the main database
fn main_database_pool(db_url: &str) -> MainDatabase {
    let manager: ConnectionManager<PgConnection> = ConnectionManager::new(db_url);
    let pool = Pool::builder()
        .connection_timeout(std::time::Duration::from_millis(500))
        .build(manager)
        .unwrap();
    MainDatabase(pool)
}
