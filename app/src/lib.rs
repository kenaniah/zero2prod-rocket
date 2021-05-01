#[macro_use]
extern crate rocket;
#[macro_use]
extern crate db;

mod routes;
use rocket::{data::Outcome, request::{self, Request, FromRequest}};
use routes::{health_check, subscriptions};

use db::{PgConnection, r2d2::{ConnectionManager, Pool, PooledConnection}};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub struct MainDatabase(PgPool);

impl<'a, 'r> FromRequest<'r> for MainDatabase {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let pool = request.guard::<Self>();
        Outcome::Success(pool.await)
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
