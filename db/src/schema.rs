table! {
    subscriptions (id) {
        id -> Int4,
        email -> Text,
        name -> Text,
        subscribed_at -> Timestamptz,
    }
}
