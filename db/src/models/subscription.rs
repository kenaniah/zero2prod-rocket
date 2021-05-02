#[derive(Queryable, Debug)]
pub struct Subscription {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub subscribed_at: chrono::NaiveDateTime,
}
