use actix::{Actor, Addr, SyncArbiter, SyncContext};
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};

pub mod schema;
pub mod models;

pub struct AppState {
    pub db: Addr<DbActor>,
}

pub struct DbActor {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

pub fn connect(url: &str) -> Addr<DbActor> {
    let pool = get_pool(url);
    SyncArbiter::start(5, move || DbActor { pool: pool.clone() })
}

fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Error building a pool")
}
