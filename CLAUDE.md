# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust-based backend application using a Cargo workspace with three members:
- **server**: Main Axum-based REST API server (Rust 2024 edition)
- **agent**: Windows agent application (cross-compiled to x86_64-pc-windows-gnu)
- **shared**: Shared library code used by both server and agent

The server provides authentication, user management, team management (with companies and invitations), audit management, Prometheus metrics, and status endpoints. It uses HashiCorp Vault for secrets management, PostgreSQL for persistence, and Resend for email delivery.

## Build & Development Commands

### Local Development
```bash
# Build all workspace members
cargo build

# Build with release optimizations
cargo build --release

# Run the server locally (requires .env file with Vault credentials)
cargo run -p server

# Run with verbose logging
RUST_LOG=debug cargo run -p server

# Build Windows agent specifically
cargo build --target x86_64-pc-windows-gnu -p agent

# Auto-reload on file changes (requires: cargo install cargo-watch)
cargo watch -x "run -p server"

# Auto-reload with debug logging
RUST_LOG=debug cargo watch -x "run -p server"

# Run tests before reloading
cargo watch -x test -x "run -p server"
```

### Testing & Linting
```bash
# Run all tests
cargo test

# Run tests with coverage (requires PostgreSQL running)
./scripts/execute-tests.sh

# Run tests with coverage manually
LOCAL_DATABASE_URL="postgresql://test:password@localhost:5432/tests" cargo llvm-cov --all-features -p server --bin server

# Run tests with coverage for CI (uses different database URL)
cargo llvm-cov --all-features -p server --bin server --lcov --output-path coverage/lcov.info

# Run a specific test
cargo test <test_name>

# Run tests for a specific module
cargo test --test <module_name>

# Run tests with detailed output
RUST_LOG=debug cargo test -- --nocapture

# Check code without building
cargo check

# Run Clippy with pedantic lints (matches CI requirements)
cargo clippy --workspace --all-features -- -W clippy::pedantic -Dwarnings

# Run Clippy for a specific package
cargo clippy -p server -- -W clippy::pedantic -Dwarnings

# Format code
cargo fmt
```

### Database Management
```bash
# Navigate to server directory first
cd server

# Run migrations (requires DATABASE_URL environment variable)
# Set DATABASE_URL first: export DATABASE_URL="postgresql://admin:admin@localhost:5432/admin"
diesel migration run

# Create a new migration
diesel migration generate <migration_name>

# Revert last migration
diesel migration revert

# List migration status
diesel migration list

# Regenerate schema.rs from database
diesel print-schema > src/schema.rs
```

### Docker
```bash
# Build Docker image (builds both server and Windows agent)
docker build -t app:latest .

# Run with docker-compose (PostgreSQL database)
docker compose up -d

# Connect to local database (development)
psql postgresql://admin:admin@localhost:5432/admin

# Connect via Docker
docker compose exec db psql -U admin -d admin

# Stop and remove containers
docker compose down

# Stop and remove with volumes (clears all data)
docker compose down -v
```

## Architecture

### Workspace Structure
The project uses Cargo workspaces defined in root `Cargo.toml` with optimized release profiles (LTO enabled, single codegen unit for size optimization).

### Server Architecture

The server follows a modular architecture with feature-based organization:

#### Application Bootstrap (`main.rs` → `utils.rs`)
1. Logger initialization via `tracing-subscriber`
2. Secrets fetched from Vault via AppRole authentication
3. `GlobalState` initialized with database pool, Resend client, RNG, and CSRF secret
4. Prometheus metrics recorder installed
5. Router assembled with CORS, tracing, cookie, and CSRF protection layers
6. Server binds to `0.0.0.0:8080`

#### Global State (`state.rs`)
Shared application state containing:
- **Database connection pool**: Diesel r2d2 pool for PostgreSQL (max 10 connections)
- **Resend client**: Email service client wrapped in Arc<Mutex>
- **RNG**: Seeded random number generator for cryptographic operations
- **Secrets**: Vault-retrieved configuration (database URL, JWT secret, CORS origins, API keys)
- **CSRF secret**: Derived from JWT secret for CSRF token generation

Notes:
- JWT_SECRET is unsafely set as environment variable to support the auth middleware which cannot access GlobalState directly
- Migrations are automatically run on startup
- Test constructor `new_with_pool()` available for testing with deterministic RNG seed

#### Module Organization Pattern
Each feature module (auth, users, teams, status) is located in `server/src/api/` and follows this structure:
- `mod.rs`: Public API exports
- `endpoints.rs`: Axum route handlers with OpenAPI documentation (utoipa)
- `services.rs`: Business logic and `get_routes()` function
- `models.rs` or `models/`: Data structures (DB models, payloads, responses)
- `error.rs`: Feature-specific error types (using thiserror)

Nested modules (e.g., `teams/companies/`, `teams/invitations/`) follow the same pattern.

#### Authentication & Security
**JWT Authentication:**
- JWT-based authentication with cookie storage
- Middleware layers: `middleware()` (base auth), `admin_middleware()`, `pentester_middleware()`
- Password hashing uses double SHA-256 (via `sha2` crate)
- JWT claims include user ID, username, email, and role
- Password reset flow uses time-limited tokens stored in database

**CSRF Protection:**
- Custom CSRF middleware applied before cookie layer (order is critical)
- Generates HMAC-based tokens using CSRF secret derived from JWT secret
- Token stored in cookie and validated via `x-csrf-token` header
- Bypasses GET requests and certain safe routes

#### Secrets Management
Vault integration using AppRole authentication:
1. `.env` file contains `VAULT_ADDRESS`, `VAULT_ROLE_ID`, `VAULT_SECRET_ID`
2. `VAULT_ENV` determines environment (local/dev/prod)
3. Secrets read from `secret/env/{env}/app` path
4. Required secrets: DATABASE_URL, JWT_SECRET, STATUS, STATUS_MESSAGE, ORIGIN_CORS, RESEND_API_KEY

#### API Structure
All routes are prefixed with `/v1`:
- `/v1/auth` - Login, register, password reset, email confirmation, logout
- `/v1/users` - User management and profiles
- `/v1/teams` - Team CRUD operations, member management
  - `/v1/teams/{team_id}/invitations` - Team invitation management
  - `/v1/teams/{team_id}/company` - Company CRUD within teams (SIRET-based)
- `/v1/audits` - Audit management (pentest audits linked to teams and companies)
- `/v1/status` - Server status endpoints
- `/metrics` - Prometheus metrics endpoint (available in all environments)
- `/v1/swagger` - OpenAPI documentation (disabled in production)

Swagger UI available at `/v1/swagger` when `VAULT_ENV != "prod"`.

**Layer Ordering (outer to inner):**
1. CORS layer
2. Tracing layer
3. Cookie manager layer
4. CSRF protection middleware
5. Route-specific auth middleware (where applicable)

### Database Schema
Uses Diesel ORM with PostgreSQL:
- **users**: User accounts with roles (admin/pentester/user), avatar URLs
- **teams**: Team entities with name and description
- **teams_members**: Many-to-many relationship with roles (owner/member)
- **teams_invitations**: Token-based team invitations with expiration and usage limits
- **companies**: Company information linked to teams (SIRET-validated)
- **audits**: Audit records linked to teams and companies, with archiving support
- **audits_clients**: Many-to-many relationship between audits and client users
- **password_reset_tokens**: Time-limited reset tokens

Schema defined in `server/src/schema.rs` (auto-generated by Diesel).

**Important:** Migrations run automatically on server startup via `diesel_migrations::run_pending_migrations_on_startup()`.

### CI/CD Pipeline
GitLab CI with 6 stages:
1. **notify**: Discord notifications for merge request reviewers
2. **test**: `cargo check` → `cargo clippy` (pedantic + warnings as errors) → `cargo test` with coverage
3. **sonar**: SonarQube analysis with coverage and clippy reports
4. **build**: Docker image creation with Buildah
5. **deploy**: Automated deployment to dev/prod registries
6. **mirror**: GitHub mirror using git subtree (main branch only)

Test stage uses PostgreSQL 17 service and generates LCOV coverage reports converted to SonarQube XML format. Main branch triggers both dev and prod builds. Clippy must pass with `-W clippy::pedantic -Dwarnings`.

CI enforces Rust 1.91.0 via `rust:1.91` Docker image.

## Development Guidelines

### Code Style
- Edition 2024 with Rust 1.91.0 minimum (local may have 1.91.1, CI enforces 1.91.0)
- Clippy pedantic lints enforced (`#[lints.clippy] pedantic = "warn"` in Cargo.toml)
- Allow `clippy::needless_for_each` globally in main.rs
- Use `thiserror` for custom error types
- Use `utoipa` macros for OpenAPI documentation
- Use `validator` crate with derive macros for payload validation

### Testing Infrastructure
The project has comprehensive test coverage located in `server/src/tests/`:
- **Unit tests**: Within feature modules (`auth`, `users`, `teams`)
- **Integration tests**: In `tests/integration/` using `axum-test` for HTTP testing
- **Test helpers**: Shared utilities in `tests/helpers/`
  - `db.rs`: Database pool management with testcontainers
  - `app.rs`: Test router and state creation
  - `fixtures.rs`: Common test data generators
  - `auth.rs`: Authentication helpers for tests

Tests use testcontainers for isolated PostgreSQL instances or CI database service (controlled by `CI_DATABASE` env var).

**Database URLs:**
- Development: `postgresql://admin:admin@localhost:5432/admin`
- Tests (local): `postgresql://test:password@localhost:5432/tests` (set via `LOCAL_DATABASE_URL`)
- Tests (CI): `postgres://testuser:testpass@postgres:5432/testdb` (set via `DATABASE_URL`)

Test dependencies include: `testcontainers`, `axum-test`, `mockall`, `serial_test`, `diesel_migrations`.

**Running Tests:**
- Tests require PostgreSQL running via `docker compose up -d`
- Use `./scripts/execute-tests.sh` for local testing with coverage
- Tests use `#[serial_test]` attribute where database state needs isolation
- Integration tests use `axum-test` crate for HTTP testing
- Test helpers provide utilities: `create_test_router()`, `create_test_state()`, auth helpers, fixtures

**Writing New Tests:**
1. Unit tests: Add to `src/tests/<module>/<feature>_tests.rs`
2. Integration tests: Add to `src/tests/integration/<module>.rs`
3. Use test helpers from `src/tests/helpers/` (db, app, fixtures, auth)
4. Mock external services (e.g., Resend) using `mockall` when needed
5. Use `#[serial_test]` for tests that modify shared database state

### Environment Variables
Development requires:
- `VAULT_ADDRESS`: Vault server URL
- `VAULT_ROLE_ID`: AppRole role ID
- `VAULT_SECRET_ID`: AppRole secret ID
- `VAULT_ENV`: Environment name (local/dev/prod)

### Adding New Endpoints
1. Define request/response models with `#[derive(Deserialize, Serialize)]` and utoipa macros
2. Create handler in `endpoints.rs` with `#[utoipa::path]` attribute
3. Add business logic in `services.rs`
4. Register route in module's `get_routes()` function
5. Update `swagger/mod.rs` ApiDoc if needed

### Database Changes
1. Create migration: `cd server && diesel migration generate <name>`
2. Write `up.sql` and `down.sql` in the generated migration directory
3. Run migration: `diesel migration run` (requires `DATABASE_URL` env var)
4. Regenerate schema: `diesel print-schema > src/schema.rs`
5. Update models in relevant module
6. Run tests to verify: `./scripts/execute-tests.sh`

Note: Migrations are auto-applied on server startup, so manual running is only needed for development/testing.

### Monitoring & Observability

**Prometheus Metrics:**
- Metrics exported via `/metrics` endpoint (available in all environments)
- Recorder initialized on startup via `metrics::install_recorder()`
- Uses `metrics` crate with `metrics-exporter-prometheus` backend
- Standard HTTP metrics automatically tracked by middleware

**Logging:**
- Uses `tracing` and `tracing-subscriber` for structured logging
- Default log level: INFO (configurable via `RUST_LOG` env var)
- HTTP request/response logging via tower-http TraceLayer
- Set `RUST_LOG=debug` for verbose output during development

### Development Workflow

**Initial Setup:**
1. Create `.env` file with Vault credentials (VAULT_ADDRESS, VAULT_ROLE_ID, VAULT_SECRET_ID, VAULT_ENV=local)
2. Start PostgreSQL: `docker compose up -d`
3. Set DATABASE_URL: `export DATABASE_URL="postgresql://admin:admin@localhost:5432/admin"`
4. Run migrations: `cd server && diesel migration run && cd ..`
5. Start server: `cargo run -p server`

**Making Changes:**
1. If adding new routes: Update module's `endpoints.rs`, `services.rs`, and `get_routes()` function
2. If changing database: Create migration → run migration → regenerate schema → update models
3. Run tests before committing: `./scripts/execute-tests.sh`
4. Run clippy: `cargo clippy --workspace --all-features -- -W clippy::pedantic -Dwarnings`
5. Format code: `cargo fmt`

**Accessing the Application:**
- Server: `http://localhost:8080/v1/`
- Swagger UI: `http://localhost:8080/v1/swagger/` (local/dev only)
- Metrics: `http://localhost:8080/metrics`
- Status: `http://localhost:8080/v1/status`

### Cross-Compilation
The Windows agent is built during Docker image creation using mingw-w64 toolchain. The Dockerfile handles all cross-compilation setup.
