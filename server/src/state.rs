use crate::vault::Secrets;
use base64::{Engine, engine::general_purpose::STANDARD};
use chrono::{DateTime, Utc};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use rand::{SeedableRng, rngs::StdRng};
use resend_rs::Resend;
use socketioxide::SocketIo;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, OnceLock};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;
use tracing::info;

/// Handle to a connected agent, stored in the agent registry.
#[allow(dead_code)]
pub struct AgentHandle {
    pub agent_id: String,
    pub audit_id: String,
    pub addr: SocketAddr,
    pub shutdown: tokio::sync::watch::Sender<bool>,
    pub last_heartbeat: Arc<std::sync::Mutex<Option<DateTime<Utc>>>>,
    pub latency_ms: Arc<std::sync::Mutex<Option<u64>>>,
    pub shell_stream_id: Arc<std::sync::Mutex<Option<u32>>>,
    pub shell_cwd: Arc<std::sync::Mutex<Option<String>>>,
    pub system_info: Arc<std::sync::Mutex<Option<shared::packets::SystemInfo>>>,
    pub writer: Arc<Mutex<OwnedWriteHalf>>,
    pub shared_secret: [u8; 32],
}

impl AgentHandle {
    /// Signal the agent to disconnect.
    pub fn shutdown(&self) {
        let _ = self.shutdown.send(true);
    }

    /// Send an encrypted packet to the agent via the TCP writer.
    ///
    /// # Errors
    /// Returns an error if encryption or writing fails.
    #[allow(dead_code)]
    pub async fn send_packet(
        &self,
        data: &[u8],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        use shared::encryption::encrypt;
        use tokio::io::AsyncWriteExt;

        let (encrypted, nonce) = encrypt(&self.shared_secret, data)?;

        #[allow(clippy::cast_possible_truncation)]
        let len = encrypted.len() as u32;
        let total_size = 4 + nonce.len() + encrypted.len();
        let mut buf = vec![0u8; total_size];

        buf[0..4].copy_from_slice(&len.to_be_bytes());
        buf[4..16].copy_from_slice(&nonce);
        buf[16..].copy_from_slice(&encrypted);

        let mut writer = self.writer.lock().await;
        writer.write_all(&buf).await?;
        writer.flush().await?;
        Ok(())
    }

    /// Check if the agent is online based on heartbeat recency.
    #[must_use]
    pub fn is_online(&self, timeout_secs: i64) -> bool {
        self.last_heartbeat
            .lock()
            .ok()
            .and_then(|hb| *hb)
            .is_some_and(|ts| (Utc::now() - ts).num_seconds() < timeout_secs)
    }
}

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
    pub agent_registry: Arc<std::sync::Mutex<HashMap<String, AgentHandle>>>,
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
            agent_registry: Arc::new(std::sync::Mutex::new(HashMap::new())),
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

    /// Get an optional reference to the [`SocketIo`] handle.
    /// Returns `None` if not yet initialized (e.g. during early startup or tests).
    pub fn io_opt(&self) -> Option<&SocketIo> {
        self.io.get()
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
            agent_registry: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }
}
