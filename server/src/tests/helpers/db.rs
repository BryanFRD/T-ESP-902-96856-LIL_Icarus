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

/// Nettoie toutes les données de la base de données (pour les tests en CI qui partagent la même DB)
#[cfg(test)]
pub fn cleanup_database(pool: &DbPool) {
    use diesel::sql_query;
    use diesel::RunQueryDsl;

    let mut conn = pool.get().expect("Failed to get database connection");

    // Utiliser une seule requête avec RESTART IDENTITY pour éviter les deadlocks
    sql_query(
        "TRUNCATE TABLE password_reset_tokens, teams_invitations, teams_members, teams, users RESTART IDENTITY CASCADE"
    )
    .execute(&mut conn)
    .expect("Failed to truncate tables");
}

/// Fonction helper qui configure tout : démarre PostgreSQL, crée le pool, exécute les migrations
/// Retourne le container PostgreSQL (Option - None en CI) et le pool de connexions
///
/// En CI (quand CI_DATABASE=true), utilise le service PostgreSQL de GitLab
/// En local, utilise testcontainers pour lancer un container PostgreSQL
#[cfg(test)]
pub async fn setup_test_db() -> (Option<ContainerAsync<Postgres>>, DbPool) {
    // Détecter si on est en CI avec un service PostgreSQL
    let use_ci_database = std::env::var("CI_DATABASE").is_ok();

    let (container, database_url) = if use_ci_database {
        // En CI: utiliser le service PostgreSQL de GitLab
        let db_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set when CI_DATABASE=true");

        // Attendre que PostgreSQL soit prêt
        wait_for_postgres(&db_url).await;

        (None, db_url)
    } else {
        // En local: utiliser testcontainers
        let (container, db_url) = setup_test_postgres().await;
        (Some(container), db_url)
    };

    let pool = create_test_pool(&database_url);
    run_migrations(&pool);

    // En CI, nettoyer la base de données avant chaque test (car elle est partagée)
    if use_ci_database {
        cleanup_database(&pool);
    }

    (container, pool)
}

/// Attendre que PostgreSQL soit prêt (pour les services CI)
#[cfg(test)]
async fn wait_for_postgres(database_url: &str) {
    use std::time::Duration;
    use tokio::time::sleep;

    for attempt in 1..=30 {
        match create_test_pool(database_url).get() {
            Ok(_) => {
                println!("PostgreSQL is ready!");
                return;
            }
            Err(e) => {
                if attempt == 30 {
                    panic!("PostgreSQL not ready after 30 attempts: {}", e);
                }
                println!("Waiting for PostgreSQL... (attempt {}/30)", attempt);
                sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
