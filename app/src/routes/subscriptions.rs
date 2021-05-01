use crate::MainConnection;
use diesel::RunQueryDsl;
use rocket::form::Form;

use db::schema::subscriptions;
use db::models::Subscription;

#[derive(FromForm, Insertable)]
#[table_name = "subscriptions"]
pub struct FormData {
    name: String,
    email: String,
}

#[post("/", data = "<data>")]
pub fn subscribe(db: MainConnection, data: Form<FormData>) {
    let _res: Subscription = diesel::insert_into(subscriptions::table)
        .values(data.into_inner())
        .get_result(&*db)
        .unwrap();
}
