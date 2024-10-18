use diesel::{r2d2::{ConnectionManager, Pool}, PgConnection};


pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
}