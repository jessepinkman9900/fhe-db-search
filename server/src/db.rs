use crate::config::Config;
use database::models::NewKV;
use database::models::KV;
use database::schema::kv::dsl as kvdsl;
use diesel::{prelude::*, r2d2::ConnectionManager, r2d2::PooledConnection};
use r2d2::Pool;

pub fn connect_db(config: &Config) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(config.db_url());
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create db pool")
}

pub async fn get_db_connection(
    pool: &Pool<ConnectionManager<PgConnection>>,
) -> Result<PooledConnection<ConnectionManager<PgConnection>>, String> {
    pool.get()
        .map_err(|e| format!("Failed to get db connection: {}", e))
}

pub async fn create_kv(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
    client_id: String,
    key: String,
    value: String,
) -> KV {
    let new_row = NewKV {
        client_id,
        key,
        value,
    };
    diesel::insert_into(kvdsl::kv)
        .values(&new_row)
        .returning(KV::as_returning())
        .get_result(connection)
        .expect("Failed to insert row")
}

pub async fn get_kvs(
    connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
) -> Vec<KV> {
    kvdsl::kv
        .select(KV::as_select())
        .order_by(kvdsl::id.desc())
        .load(connection)
        .expect("Failed to load rows")
}
