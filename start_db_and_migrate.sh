#!/usr/bin/env bash

set -e

# Configuration
CONTAINER_NAME="temp-postgres"
DB_USER="postgres"
DB_PASSWORD="password"
DB_NAME="workspace-kit"
DB_PORT=5432
DATABASE_URL="postgresql://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"

# Start PostgreSQL in Docker with auto-remove
echo "Starting PostgreSQL Docker container..."
docker run --rm \
  --name "$CONTAINER_NAME" \
  -e POSTGRES_USER="$DB_USER" \
  -e POSTGRES_PASSWORD="$DB_PASSWORD" \
  -e POSTGRES_DB="$DB_NAME" \
  -p "$DB_PORT":5432 \
  postgres &

# Save Docker process ID
DOCKER_PID=$!

# Cleanup on exit
function cleanup {
  echo "Stopping PostgreSQL container..."
  kill $DOCKER_PID
}
trap cleanup EXIT

# Wait for Postgres to be ready
echo "Waiting for PostgreSQL to be ready..."
until pg_isready -h localhost -p "$DB_PORT" -U "$DB_USER" > /dev/null 2>&1; do
  sleep 1
done

# Set the DATABASE_URL for sqlx
export DATABASE_URL="$DATABASE_URL"

# Run migrations
echo "Running sqlx migrations..."
sqlx migrate run

echo "Migrations complete. Press Ctrl+C to stop the database."

# Wait for user to terminate
wait
