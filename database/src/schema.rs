// @generated automatically by Diesel CLI.

diesel::table! {
    kv (id) {
        id -> Int4,
        #[max_length = 255]
        client_id -> Varchar,
        #[max_length = 255]
        key -> Varchar,
        #[max_length = 255]
        value -> Varchar,
    }
}
