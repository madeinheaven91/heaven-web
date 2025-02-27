use actix::{Actor, Addr, SyncContext};
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder().build(manager).expect("Error building a pool")
}

pub struct AppState{
    pub db: Addr<DbActor>
}

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}
