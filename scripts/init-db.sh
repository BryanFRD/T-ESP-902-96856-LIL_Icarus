#!/bin/bash
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    -- Create dev user and database
    CREATE USER "user" WITH PASSWORD 'password';
    CREATE DATABASE db OWNER "user";
    GRANT ALL PRIVILEGES ON DATABASE db TO "user";

    -- Create test user and database
    CREATE USER test WITH PASSWORD 'password';
    CREATE DATABASE tests OWNER test;
    GRANT ALL PRIVILEGES ON DATABASE tests TO test;
EOSQL

echo "Databases and users created successfully"
