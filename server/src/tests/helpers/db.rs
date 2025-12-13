#[cfg(test)]
use diesel::r2d2::{self, ConnectionManager};
#[cfg(test)]
use diesel::PgConnection;
#[cfg(test)]
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
#[cfg(test)]
use std::sync::Arc;
#[cfg(test)]
use testcontainers::ContainerAsync;
#[cfg(test)]
use testcontainers::runners::AsyncRunner;
#[cfg(test)]
use testcontainers_modules::postgres::Postgres;

#[cfg(test)]
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[cfg(test)]
pub type DbPool = Arc<r2d2::Pool<ConnectionManager<PgConnection>>>;

/// Configure et démarre un container PostgreSQL pour les tests
/// Retourne le container et l'URL de connexion à la base de données
#[cfg(test)]
pub async fn setup_test_postgres() -> (ContainerAsync<Postgres>, String) {
    // Créer et démarrer le container PostgreSQL de manière asynchrone
    let container = Postgres::default()
        .start()
        .await
        .expect("Failed to start PostgreSQL container");

    // Récupérer le port mappé
    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("Failed to get PostgreSQL port");

    // Construire l'URL de connexion
    // testcontainers-modules utilise "postgres" comme user et password par défaut
    let database_url = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        host_port
    );

    (container, database_url)
}

/// Crée un pool de connexions Diesel pour les tests
#[cfg(test)]
pub fn create_test_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create database pool");

    Arc::new(pool)
}

/// Exécute toutes les migrations Diesel sur la base de données de test
#[cfg(test)]
pub fn run_migrations(pool: &DbPool) {
    let mut conn = pool.get().expect("Failed to get database connection");

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
}

/// Fonction helper qui configure tout : démarre PostgreSQL, crée le pool, exécute les migrations
/// Retourne le container PostgreSQL (à garder en vie pendant les tests) et le pool de connexions
#[cfg(test)]
pub async fn setup_test_db() -> (ContainerAsync<Postgres>, DbPool) {
    let (container, database_url) = setup_test_postgres().await;
    let pool = create_test_pool(&database_url);
    run_migrations(&pool);

    (container, pool)
}
