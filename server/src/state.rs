use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use std::sync::Arc;

#[derive(Clone)]
pub struct GlobalState {
    pub db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl GlobalState {
    pub fn new(database_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Failed to create pool.");

        Self {
            db_pool: Arc::new(pool),
        }
    }

    pub fn get_db_conn(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
        self.db_pool.get()
    }
}
