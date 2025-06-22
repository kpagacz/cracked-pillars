#!/bin/sh
set -e

DB_PATH="/app/data/hammer.db3"

if [ ! -f "$DB_PATH" ]; then
  echo "Database not found, importing from quarry..."
  echo "Setting up environment for import..."
  export RUST_LOG=debug
  export RUST_BACKTRACE=1

  echo "Starting import process..."
  if ! /usr/local/bin/hammer --import-from-quarry; then
    echo "Import failed with exit code $?"
    exit 1
  fi
  echo "Import completed successfully."
else
  echo "Database found, skipping import."
fi

echo "Starting backend server..."
echo "Setting up environment for server..."
export RUST_LOG=debug
export RUST_BACKTRACE=1

echo "Server starting..."
exec /usr/local/bin/hammer
