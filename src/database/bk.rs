/*
pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager};

type DBConnection = ConnectionManager<PgConnection>;
pub type DBPool = r2d2::Pool<DBConnection>;

pub fn init_pool() -> r2d2::Pool<DBConnection> {
    let connection_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL is not added to environment");
    let manager = DBConnection::new(connection_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Unavailable connect to Data Base")
}*/
