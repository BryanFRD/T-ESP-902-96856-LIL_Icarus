# ESP Backend

Backend du projet ESP développé en Rust avec Axum et Diesel.

## Prérequis

- **Rust** : version 1.87.0 ou supérieure
  - Installation : https://rustup.rs/
- **Docker & Docker Compose** : pour la base de données PostgreSQL
- **Diesel CLI** : pour gérer les migrations de base de données
  ```bash
  cargo install diesel_cli --no-default-features --features postgres
  ```

## Installation

1. **Cloner le dépôt**
   ```bash
   git clone git@git.icarus-cyber.fr:icarus/app.git
   cd back
   ```

2. **Créer le fichier `.env`**

   Le fichier `.env` doit contenir les variables suivantes :
   ```env
   VAULT_ADDRESS=https://vault.icarus-cyber.fr
   VAULT_ROLE_ID=<votre-role-id>
   VAULT_SECRET_ID=<votre-secret-id>
   ```
   (Voir [Outline](https://outline.icarus-cyber.fr/doc/vault-QUUZy8xVlr) pour les valeurs)

3. **Démarrer la base de données PostgreSQL**
   ```bash
   docker compose up -d
   ```

   Cela démarre une instance PostgreSQL avec :
   - User: `user`
   - Password: `password`
   - Database: `db`
   - Port: `5432`

4. **Configurer la variable d'environnement pour Diesel**

   Définir l'URL de la base de données pour Diesel :
   ```bash
   # Windows (PowerShell)
   $env:DATABASE_URL="postgres://user:password@localhost:5432/db"

   # Linux/Mac
   export DATABASE_URL="postgres://user:password@localhost:5432/db"
   ```

5. **Exécuter les migrations de base de données**
   ```bash
   cd server
   diesel migration run
   cd ..
   ```

6. **Compiler le projet**
   ```bash
   cargo build
   ```

## Lancer le projet en développement

### Démarrer le serveur

```bash
cargo run --bin server
```

Le serveur démarre sur `http://0.0.0.0:80` (nécessite les privilèges administrateur sur certains systèmes).

### Démarrer l'agent

```bash
cargo run -p agent
```

### Mode watch (recompilation automatique)

Installer `cargo-watch` :
```bash
cargo install cargo-watch
```

Puis lancer en mode watch :
```bash
cargo watch -x "run --bin server"
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
cargo clippy
```

Le projet utilise `clippy` en mode `pedantic` (voir `server/Cargo.toml`).

#### Formater le code
```bash
cargo fmt
```

#### Exécuter les tests
```bash
cargo test
```

#### Compiler en mode release
```bash
cargo build --release
```

Les binaires optimisés seront dans `target/release/`.

### Gestion de la base de données

#### Créer une nouvelle migration
```bash
cd server
diesel migration generate <nom_de_la_migration>
```

#### Exécuter les migrations
```bash
diesel migration run
```

#### Annuler la dernière migration
```bash
diesel migration revert
```

#### Vérifier le statut des migrations
```bash
diesel migration list
```

### Documentation API (Swagger)

Une fois le serveur démarré, la documentation OpenAPI/Swagger est accessible via :
```
http://localhost/v1/swagger/
```