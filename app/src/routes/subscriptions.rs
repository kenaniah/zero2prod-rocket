use crate::MainConnection;
use diesel::RunQueryDsl;
use rocket::form::{self, Form, FromForm};

use db::models::Subscription;
use db::schema::subscriptions;

#[derive(FromForm, Insertable, Debug)]
#[table_name = "subscriptions"]
pub struct FormData {
    #[field(validate = validate_name())]
    name: String,
    email: String,
}

#[post("/", data = "<data>")]
pub fn subscribe(db: MainConnection, data: Form<FormData>) {
    let _res: Subscription = diesel::insert_into(subscriptions::table)
        .values(&*data)
        .get_result(&*db)
        .unwrap();
}

fn validate_name<'v>(name: &str) -> form::Result<'v, ()> {
    let is_empty_or_whitespace = name.trim().is_empty();
    let is_too_long = name.chars().count() > 256;
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = name.chars().any(|g| forbidden_characters.contains(&g));
    if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
        return Err(form::Error::validation("invalid name").into());
    }
    Ok(())
}
