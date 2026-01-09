.PHONY: test test-fast test-slow coverage coverage-fast

# Variables
LOCAL_DB_URL := postgresql://user:password@localhost:5432/db

# Tests rapides avec base de données partagée (8 secondes)
test-fast:
	@echo "Running tests with shared database (fast mode)..."
	LOCAL_DATABASE_URL=$(LOCAL_DB_URL) cargo llvm-cov --all-features -p server --bin server

# Tests avec testcontainers (lent mais isolé)
test-slow:
	@echo "Running tests with testcontainers (slow mode)..."
	cargo llvm-cov --all-features -p server --bin server

# Alias pour test-fast (par défaut)
test: test-fast

# Coverage rapide avec génération de rapport HTML
coverage-fast:
	@echo "Running tests with coverage report (fast mode)..."
	LOCAL_DATABASE_URL=$(LOCAL_DB_URL) cargo llvm-cov --all-features -p server --bin server --html
	@echo "Coverage report generated in target/llvm-cov/html/index.html"

# Alias pour coverage-fast (par défaut)
coverage: coverage-fast

# Build le projet
build:
	cargo build

# Build en release
build-release:
	cargo build --release

# Check le code sans compiler
check:
	cargo check

# Clippy avec lints pedantic
clippy:
	cargo clippy -- -W clippy::pedantic -Dwarnings

# Clean le projet
clean:
	cargo clean

# Aide
help:
	@echo "Commandes disponibles:"
	@echo "  make test          - Lance les tests en mode rapide (base partagée) [défaut]"
	@echo "  make test-fast     - Lance les tests avec base de données partagée (~8s)"
	@echo "  make test-slow     - Lance les tests avec testcontainers (~5min)"
	@echo "  make coverage      - Génère le rapport de couverture en mode rapide [défaut]"
	@echo "  make coverage-fast - Génère le rapport de couverture avec base partagée"
	@echo "  make build         - Compile le projet"
	@echo "  make build-release - Compile le projet en mode release"
	@echo "  make check         - Vérifie le code sans compiler"
	@echo "  make clippy        - Lance clippy avec lints pedantic"
	@echo "  make clean         - Nettoie le projet"
	@echo ""
	@echo "Note: Assurez-vous que PostgreSQL tourne sur localhost:5432 pour les modes rapides"
	@echo "      (user: user, password: password, database: db)"
