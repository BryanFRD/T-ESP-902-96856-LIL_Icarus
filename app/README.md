# ESP Backend

Backend du projet ESP développé en Rust avec Axum et Diesel.

## Prérequis

- **Rust** : version 1.91.0 ou supérieure
  - Installation : https://rustup.rs/
  - Vérifier la version installée : `rustc --version`
- **Docker & Docker Compose** : pour la base de données PostgreSQL
  - Docker : https://docs.docker.com/get-docker/
  - Docker Compose est inclus avec Docker Desktop
- **Diesel CLI** : pour gérer les migrations de base de données
  ```bash
  cargo install diesel_cli --no-default-features --features postgres
  ```
- **Accès à Vault** : pour récupérer les secrets (DATABASE_URL, JWT_SECRET, etc.)
  - Demander les credentials à l'équipe (VAULT_ROLE_ID et VAULT_SECRET_ID)

## Installation

1. **Cloner le dépôt**
   ```bash
   git clone git@git.icarus-cyber.fr:icarus/app.git
   cd back
   ```

2. **Créer le fichier `.env`**

   À la racine du projet `back/`, créer un fichier `.env` avec les variables suivantes :
   ```env
   VAULT_ADDRESS=https://vault.icarus-cyber.fr
   VAULT_ROLE_ID=<votre-role-id>
   VAULT_SECRET_ID=<votre-secret-id>
   VAULT_ENV=local
   ```

   **Important :**
   - Les valeurs `VAULT_ROLE_ID` et `VAULT_SECRET_ID` sont disponibles sur [Outline](https://outline.icarus-cyber.fr/doc/vault-QUUZy8xVlr)
   - `VAULT_ENV` peut être `local`, `dev` ou `prod` selon l'environnement
   - En développement local, utiliser `VAULT_ENV=local`

3. **Démarrer la base de données PostgreSQL**

   À la racine du projet, lancer Docker Compose :
   ```bash
   docker compose up -d
   ```

   Cela démarre une instance PostgreSQL avec les paramètres suivants :
   - **User** : `user`
   - **Password** : `password`
   - **Database** : `db`
   - **Port** : `5432`
   - **Host** : `localhost`

   Vérifier que le conteneur est bien démarré :
   ```bash
   docker compose ps
   ```

4. **Configurer la variable d'environnement pour Diesel**

   Diesel a besoin de connaître l'URL de la base de données pour gérer les migrations.

   **Linux/Mac** :
   ```bash
   export DATABASE_URL="postgresql://user:password@localhost:5432/db"
   ```

   **Windows (PowerShell)** :
   ```powershell
   $env:DATABASE_URL="postgresql://user:password@localhost:5432/db"
   ```

   **Windows (CMD)** :
   ```cmd
   set DATABASE_URL=postgresql://user:password@localhost:5432/db
   ```

   > **Astuce** : Pour éviter de définir cette variable à chaque session, vous pouvez l'ajouter à votre fichier de configuration shell (`.bashrc`, `.zshrc`, etc.) ou créer un fichier `.env.local` et utiliser `source .env.local`.

5. **Exécuter les migrations de base de données**

   Les migrations créent toutes les tables nécessaires dans la base de données :
   ```bash
   cd server
   diesel migration run
   cd ..
   ```

   Vous devriez voir la liste des migrations appliquées. Pour vérifier :
   ```bash
   cd server
   diesel migration list
   cd ..
   ```

6. **Compiler le projet**

   Compiler tous les composants du workspace (server, agent, shared) :
   ```bash
   cargo build
   ```

   Cette étape peut prendre quelques minutes la première fois car Cargo télécharge et compile toutes les dépendances.

## Lancer le projet en développement

**Prérequis avant de lancer :**
1. ✅ Docker Compose est lancé (`docker compose ps` doit montrer PostgreSQL)
2. ✅ Les migrations sont appliquées (`diesel migration run`)
3. ✅ Le fichier `.env` existe avec les variables Vault

### Démarrer le serveur

```bash
cargo run -p server
```

Le serveur démarre sur `http://0.0.0.0:8080`.

Une fois démarré, vous pouvez accéder à :
- **API** : `http://localhost:8080/v1/`
- **Swagger UI** : `http://localhost:8080/v1/swagger/` (uniquement en mode `local` ou `dev`)
- **Status** : `http://localhost:8080/v1/status`

**Logs** : Le serveur utilise `tracing` pour les logs. Le niveau de log peut être configuré via la variable d'environnement `RUST_LOG` :
```bash
RUST_LOG=debug cargo run -p server
```

### Démarrer l'agent (Windows)

L'agent est conçu pour Windows et nécessite une cross-compilation :

```bash
cargo build --target x86_64-pc-windows-gnu -p agent
```

Pour exécuter l'agent sur Windows :
```bash
cargo run -p agent
```

### Mode watch (recompilation automatique)

Pour le développement, il est recommandé d'utiliser `cargo-watch` pour recompiler automatiquement à chaque modification de fichier.

**Installation** :
```bash
cargo install cargo-watch
```

**Utilisation** :
```bash
# Relance le serveur à chaque modification
cargo watch -x "run -p server"

# Avec des logs détaillés
RUST_LOG=debug cargo watch -x "run -p server"

# Relance uniquement si les tests passent
cargo watch -x test -x "run -p server"
```

## Développement

### Structure du projet

```
back/
├── server/          # Serveur API principal
│   ├── src/
│   │   ├── main.rs
│   │   ├── auth/     # Authentification
│   │   ├── users/    # Gestion des utilisateurs
│   │   ├── teams/    # Gestion des équipes
│   │   ├── mails/    # Envoi d'emails
│   │   └── ...
│   ├── migrations/   # Migrations Diesel
│   └── templates/    # Templates d'emails
├── agent/           # Agent
├── shared/          # Code partagé
└── compose.yml      # Docker Compose pour PostgreSQL
```

### Commandes utiles

#### Linter le code
```bash
# Clippy standard
cargo clippy

# Clippy avec les mêmes règles que la CI (pedantic + warnings as errors)
cargo clippy --workspace --all-features -- -W clippy::pedantic -Dwarnings
```

Le projet utilise `clippy` en mode `pedantic` (voir `server/Cargo.toml`).

#### Formater le code
```bash
cargo fmt
```

#### Exécuter les tests

**Prérequis** :
1. La base de données PostgreSQL doit être lancée avec Docker Compose :
   ```bash
   docker compose up -d
   ```

2. Lancer le script de tests :
   ```bash
   ./scripts/execute-tests.sh
   ```

   Ce script :
   - Configure automatiquement l'URL de la base de données de test
   - Lance tous les tests avec coverage
   - Génère un rapport de couverture

**Tests sans coverage** :
```bash
cargo test
```

**Tests avec logs détaillés** :
```bash
RUST_LOG=debug cargo test -- --nocapture
```

#### Vérifier le code
```bash
cargo check
```

#### Compiler en mode release
```bash
cargo build --release
```

Les binaires optimisés seront dans `target/release/`.

### Gestion de la base de données

**Important** : Toutes les commandes Diesel doivent être exécutées depuis le répertoire `server/` avec la variable `DATABASE_URL` définie.

#### Créer une nouvelle migration

```bash
cd server
diesel migration generate <nom_de_la_migration>
```

Cela crée deux fichiers dans `server/migrations/<timestamp>_<nom>/` :
- `up.sql` : Le SQL à exécuter pour appliquer la migration
- `down.sql` : Le SQL à exécuter pour annuler la migration

Exemple :
```bash
cd server
diesel migration generate create_companies
```

#### Exécuter les migrations

Appliquer toutes les migrations non encore exécutées :
```bash
cd server
diesel migration run
```

#### Annuler la dernière migration

```bash
cd server
diesel migration revert
```

#### Vérifier le statut des migrations

Liste toutes les migrations et leur statut (appliquée ou non) :
```bash
cd server
diesel migration list
```

#### Régénérer le schema.rs

Après avoir modifié la structure de la base de données (via migrations), régénérer le fichier `schema.rs` :
```bash
cd server
diesel print-schema > src/schema.rs
```

Ce fichier est utilisé par Diesel pour le typage des requêtes SQL.

#### Se connecter à la base de données

Pour inspecter manuellement la base de données :
```bash
# Via psql
psql postgresql://user:password@localhost:5432/db

# Via Docker
docker compose exec postgres psql -U user -d db
```

### Documentation API (Swagger)

Une fois le serveur démarré en mode `local` ou `dev`, la documentation OpenAPI/Swagger est accessible via :
```
http://localhost:8080/v1/swagger/
```

**Note** : Swagger est désactivé en mode `prod` (quand `VAULT_ENV=prod`).

La documentation inclut :
- Tous les endpoints de l'API avec leurs paramètres
- Les schémas de requêtes et réponses
- La possibilité de tester les endpoints directement depuis l'interface

## Troubleshooting

### Erreur "Failed to bind to address"

Si vous obtenez cette erreur au lancement du serveur, cela signifie que le port 8080 est déjà utilisé.

**Solutions** :
- Vérifier qu'aucun autre serveur ne tourne sur le port 8080
- Ou modifier le port dans `server/src/main.rs` (ligne 21)

### Erreur "connection refused" lors des tests

**Causes possibles** :
1. PostgreSQL n'est pas lancé : `docker compose up -d`
2. La base de données de test n'existe pas : vérifier que `init-db.sh` a été exécuté
3. Mauvaise URL de connexion : vérifier la variable `DATABASE_URL`

### Erreur "table does not exist"

Les migrations n'ont pas été appliquées.

**Solution** :
```bash
cd server
diesel migration run
cd ..
```

### Erreur Vault "permission denied" ou "invalid credentials"

Les credentials Vault dans le fichier `.env` sont incorrects ou expirés.

**Solutions** :
1. Vérifier que le fichier `.env` existe et contient les bonnes valeurs
2. Demander de nouveaux credentials à l'équipe
3. Consulter [Outline](https://outline.icarus-cyber.fr/doc/vault-QUUZy8xVlr)

### Les tests échouent avec "database connection error"

**Solution** :
1. Vérifier que PostgreSQL tourne : `docker compose ps`
2. Vérifier que la base de test existe : `docker compose exec postgres psql -U test -d tests`
3. Recréer les bases de données si nécessaire :
   ```bash
   docker compose down -v
   docker compose up -d
   ```

## Architecture

Voir le fichier [CLAUDE.md](./CLAUDE.md) pour une documentation détaillée de l'architecture du projet.