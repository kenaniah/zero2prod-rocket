#[derive(Queryable)]
pub struct Subscription {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub subscribed_at: chrono::NaiveDateTime,
}
