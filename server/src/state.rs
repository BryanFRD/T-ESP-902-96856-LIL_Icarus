use crate::vault::Secrets;
use base64::{engine::general_purpose::STANDARD, Engine};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use rand::{SeedableRng, rngs::StdRng};
use resend_rs::Resend;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone)]
pub struct GlobalState {
    pub db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    pub resend: Arc<Mutex<Resend>>,
    pub rng: Arc<Mutex<StdRng>>,
    secrets: Arc<Secrets>,
    pub csrf_secret: Arc<String>,
    pub event_encryption_key: Arc<Vec<u8>>,
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

        crate::migrations::run_pending_migrations_on_startup(&pool);

        let resend = Resend::new(&secrets.resend_api_key);
        let rng = StdRng::from_os_rng();

        let csrf_secret = format!("csrf-{}", secrets.jwt_secret);

        let event_key_bytes = STANDARD
            .decode(&secrets.event_encryption_key)
            .expect("EVENT_ENCRYPTION_KEY must be valid base64");

        assert!(event_key_bytes.len() == 32, "EVENT_ENCRYPTION_KEY must be 32 bytes (256 bits)");

        Self {
            db_pool: Arc::new(pool),
            resend: Arc::new(Mutex::new(resend)),
            rng: Arc::new(Mutex::new(rng)),
            secrets: Arc::new(secrets),
            csrf_secret: Arc::new(csrf_secret),
            event_encryption_key: Arc::new(event_key_bytes),
        }
    }

    pub fn get_db_conn(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, PoolError> {
        metrics::counter!("db_queries_total").increment(1);
        self.db_pool.get()
    }

    pub fn get_secrets(&self) -> &Secrets {
        &self.secrets
    }

    pub fn get_resend(&self) -> &Mutex<Resend> {
        &self.resend
    }

    pub fn get_rand(&self) -> &Mutex<StdRng> {
        &self.rng
    }

    #[cfg(test)]
    pub fn new_with_pool(
        db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
        secrets: Secrets,
    ) -> Self {
        unsafe {
            std::env::set_var("JWT_SECRET", &secrets.jwt_secret);
        }

        let resend = Resend::new(&secrets.resend_api_key);
        let rng = StdRng::seed_from_u64(42);

        let csrf_secret = format!("csrf-{}", secrets.jwt_secret);

        let event_key_bytes = STANDARD
            .decode(&secrets.event_encryption_key)
            .expect("EVENT_ENCRYPTION_KEY must be valid base64");

        assert!(event_key_bytes.len() == 32, "EVENT_ENCRYPTION_KEY must be 32 bytes (256 bits)");

        Self {
            db_pool,
            resend: Arc::new(Mutex::new(resend)),
            rng: Arc::new(Mutex::new(rng)),
            secrets: Arc::new(secrets),
            csrf_secret: Arc::new(csrf_secret),
            event_encryption_key: Arc::new(event_key_bytes),
        }
    }
}
