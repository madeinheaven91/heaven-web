use actix::{Actor, Addr, SyncContext};
use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};

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
