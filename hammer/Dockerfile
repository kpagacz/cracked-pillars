# Build stage
FROM rustlang/rust:nightly-slim AS builder

# Set working directory
WORKDIR /app

# Install system dependencies for building
RUN apt-get update && apt-get install -y \
    sqlite3 \
    libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the Cargo files for dependency caching
COPY hammer/Cargo.toml hammer/Cargo.lock ./

RUN mkdir src

# Remove the dummy main.rs and copy the actual source code
COPY hammer/src ./src

# Clean and build the application
RUN cargo clean && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    sqlite3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create app user for security
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set working directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/hammer /app/hammer

# Copy the configuration file
COPY hammer/hammer-prod.toml ./hammer.toml

# Copy the JSON data files
COPY hammer/abilities.json hammer/items.json ./

# Create data directory for the database
RUN mkdir -p /app/data

# Copy the existing database file
# COPY hammer/hammer.db3 /app/data/hammer.db3

# Create the migrations directory
RUN mkdir -p /app/migrations

# Copy the migration files
COPY hammer/resources/db /app/migrations

# Change ownership to appuser
RUN chown -R appuser:appuser /app

# Switch to appuser
USER appuser

# Expose the port the app runs on
EXPOSE 8000

# Set the entrypoint to the binary
ENTRYPOINT ["/app/hammer"]
