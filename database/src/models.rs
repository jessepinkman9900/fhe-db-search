use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::kv)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]
pub struct KV {
    pub id: i32,
    pub client_id: String,
    pub key: String,
    pub value: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::kv)]
pub struct NewKV {
    pub client_id: String,
    pub key: String,
    pub value: String,
}
