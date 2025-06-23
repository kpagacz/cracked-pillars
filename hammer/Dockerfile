# Use the latest nightly Rust image
FROM rustlang/rust:nightly-slim

# Set working directory
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    sqlite3 \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the Cargo files for dependency caching
COPY hammer/Cargo.toml hammer/Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Remove the dummy main.rs and copy the actual source code
RUN rm src/main.rs
COPY hammer/src ./src

# Copy the configuration file
COPY hammer/hammer.toml ./hammer.toml

# Copy the JSON data files
COPY hammer/abilities.json hammer/items.json ./

# Create data directory for the database
RUN mkdir -p /app/data

# Copy the existing database file
COPY hammer/hammer.db3 /app/data/hammer.db3

# Create the migrations directory
RUN mkdir -p /app/migrations

# Copy the migration files
COPY hammer/resources/db /app/migrations

# Clean
RUN cargo clean

# Build the application
RUN cargo build --release

# Debug: List the contents of target/release to verify the binary is present
RUN ls -l target/release/

# Copy the compiled binary to /app/hammer
RUN cp target/release/hammer /app/hammer && chmod +x /app/hammer

# Expose the port the app runs on
EXPOSE 8000

# Set the entrypoint to the binary
ENTRYPOINT ["/app/hammer"]

# Set default command arguments
CMD ["--skip-db-sync"]
