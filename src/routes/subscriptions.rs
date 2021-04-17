use rocket::form::Form;
use crate::MainDatabase;

#[derive(FromForm)]
pub struct FormData {
    name: String,
    email: String
}

#[post("/", data="<data>")]
pub fn subscribe(db: MainDatabase, data: Form<FormData>) {

}
