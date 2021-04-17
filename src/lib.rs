#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::Rocket;
use rocket::figment::{value::{Map, Value}, util::map};

mod routes;
use routes::{health_check, subscriptions};

use rocket_contrib::databases::diesel;
#[database("main")]
pub struct MainDatabase(diesel::PgConnection);

pub fn app() -> Rocket {

    let db: Map<&str, Value> = map! {
        "url" => "postgres:///zero2prod".into()
    };

    let figment = rocket::Config::figment()
        .merge(("databases", map!["main" => db]));

    rocket::custom(figment)
        .attach(MainDatabase::fairing())
        .mount("/health_check", routes![health_check::health_check])
        .mount("/subscriptions", routes![subscriptions::subscribe])
}
