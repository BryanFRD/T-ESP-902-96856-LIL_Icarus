use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tracing::{error, info};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn run_pending_migrations_on_startup(pool: &Pool<ConnectionManager<PgConnection>>) {
    info!("Checking database migrations...");

    let mut conn = pool
        .get()
        .expect("Failed to get database connection for migrations");

    match conn.run_pending_migrations(MIGRATIONS) {
        Ok(migrations) => {
            if migrations.is_empty() {
                info!("Database is up to date");
            } else {
                info!("Successfully applied {} migration(s)", migrations.len());
                for migration in migrations {
                    info!("  - Applied: {}", migration);
                }
            }
        }
        Err(e) => {
            error!("Failed to run migrations: {}", e);
            panic!("Cannot start server with pending/failed migrations");
        }
    }
}
