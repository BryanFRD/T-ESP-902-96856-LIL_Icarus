use crate::vault::Secrets;
use base64::{engine::general_purpose::STANDARD, Engine};
use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    PgConnection,
};
use rand::{rngs::StdRng, SeedableRng};
use resend_rs::Resend;
use socketioxide::SocketIo;
use std::sync::{Arc, OnceLock};
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone)]
pub struct GlobalState {
    pub db_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    pub resend: Arc<Mutex<Resend>>,
    pub rng: Arc<Mutex<StdRng>>,
    secrets: Arc<Secrets>,
    pub csrf_secret: Arc<String>,
    pub audit_access_secret: Arc<String>,
    pub event_encryption_key: Arc<Vec<u8>>,
    io: Arc<OnceLock<SocketIo>>,
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
        let audit_access_secret = format!("audit-{}", secrets.jwt_secret);

        let event_key_bytes = STANDARD
            .decode(&secrets.event_encryption_key)
            .expect("EVENT_ENCRYPTION_KEY must be valid base64");

        assert!(
            event_key_bytes.len() == 32,
            "EVENT_ENCRYPTION_KEY must be 32 bytes (256 bits)"
        );

        Self {
            db_pool: Arc::new(pool),
            resend: Arc::new(Mutex::new(resend)),
            rng: Arc::new(Mutex::new(rng)),
            secrets: Arc::new(secrets),
            csrf_secret: Arc::new(csrf_secret),
            audit_access_secret: Arc::new(audit_access_secret),
            event_encryption_key: Arc::new(event_key_bytes),
            io: Arc::new(OnceLock::new()),
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

    /// Store the [`SocketIo`] handle. Must be called exactly once during startup.
    ///
    /// # Panics
    /// Panics if called more than once.
    pub fn set_io(&self, io: SocketIo) {
        self.io
            .set(io)
            .expect("SocketIo handle already initialized");
    }

    /// Get a reference to the [`SocketIo`] handle for emitting WebSocket events.
    ///
    /// # Panics
    /// Panics if called before [`set_io()`](Self::set_io).
    #[allow(dead_code)]
    pub fn get_io(&self) -> &SocketIo {
        self.io
            .get()
            .expect("SocketIo handle not initialized — call set_io() first")
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
        let audit_access_secret = format!("audit-{}", secrets.jwt_secret);

        let event_key_bytes = STANDARD
            .decode(&secrets.event_encryption_key)
            .expect("EVENT_ENCRYPTION_KEY must be valid base64");

        assert!(
            event_key_bytes.len() == 32,
            "EVENT_ENCRYPTION_KEY must be 32 bytes (256 bits)"
        );

        Self {
            db_pool,
            resend: Arc::new(Mutex::new(resend)),
            rng: Arc::new(Mutex::new(rng)),
            secrets: Arc::new(secrets),
            csrf_secret: Arc::new(csrf_secret),
            audit_access_secret: Arc::new(audit_access_secret),
            event_encryption_key: Arc::new(event_key_bytes),
            io: Arc::new(OnceLock::new()),
        }
    }
}
