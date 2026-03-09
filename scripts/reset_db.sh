#!/bin/bash
# scripts/reset_db.sh

echo "Dropping database..."
sqlx database drop -y

echo "Creating database..."
sqlx database create

echo "Running migrations..."
sqlx migrate run

echo "Database reset complete!"