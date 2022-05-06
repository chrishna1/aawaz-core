use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::Connection;
use diesel_migrations::*;
use dotenv::dotenv;
use lazy_static::lazy_static;
use log::info;
use r2d2;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        r2d2::Builder::new()
            .build(manager)
            .expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing Database");
    lazy_static::initialize(&POOL);
    let conn = get_db_connection();
    if cfg!(test) {
        conn.begin_test_transaction()
            .expect("Failed to start transaction");
    }
    embedded_migrations::run(&conn).unwrap();
}

pub fn get_db_connection() -> DbConnection {
    POOL.get().unwrap()
}
