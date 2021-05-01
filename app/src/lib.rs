#[macro_use]
extern crate rocket;
#[macro_use]
extern crate db;

mod routes;

use rocket::{request::{Request, FromRequest}};
use routes::{health_check, subscriptions};

use db::{PgConnection, r2d2::{ConnectionManager, Pool, PooledConnection}};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub struct MainDatabase(PgPool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MainDatabase {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> rocket::request::Outcome<MainDatabase, Self::Error> {
        request.guard::<Self>().await
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
