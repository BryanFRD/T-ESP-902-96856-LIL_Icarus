use crate::vault::Secrets;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use resend_rs::Resend;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct GlobalState {
    pub db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    pub resend: Arc<Resend>,
    secrets: Arc<Secrets>,
}

impl GlobalState {
    pub fn new(secrets: Secrets) -> Self {
        // Ugly way to set the JWT_SECRET but need it for the JWT middleware
        // to work. The JWT middleware cannot access the GlobalState
        // directly, so we need to set it as an environment variable.
        unsafe {
            std::env::set_var("JWT_SECRET", &secrets.jwt_secret);
        }

        let manager = ConnectionManager::<PgConnection>::new(secrets.database_url.clone());
        let pool = Pool::builder()
            .max_size(10)
            .build(manager)
            .expect("Failed to create pool.");

        info!("Successfully connected to database");

        let resend = Resend::new(&secrets.resend_api_key);

        Self {
            db_pool: Arc::new(pool),
            resend: Arc::new(resend),
            secrets: Arc::new(secrets),
        }
    }

    pub fn get_db_conn(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
        self.db_pool.get()
    }

    pub fn get_secrets(&self) -> &Secrets {
        &self.secrets
    }

    pub fn get_resend(&self) -> &Resend {
        &self.resend
    }
}
