# Use the nightly Rust image as the base image for edition 2024 support
FROM rustlang/rust:nightly-alpine AS base

# Install build dependencies including SQLite
RUN apk add --no-cache musl-dev sqlite-dev

# Create a new empty shell project
WORKDIR /app

# Copy manifests
COPY hammer/Cargo.toml hammer/Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies - this is the caching Docker layer
RUN cargo build --release

# Remove the dummy main.rs
RUN rm src/main.rs

# Copy the real source code
COPY hammer/src ./src

# Build the application
RUN cargo build --release

# Create a new stage with a minimal runtime image
FROM alpine:latest AS runtime

# Install runtime dependencies including SQLite
RUN apk add --no-cache libgcc sqlite

# Create a non-root user
RUN addgroup -g 1001 -S appgroup && \
    adduser -u 1001 -S appuser -G appgroup

# Copy the binary from the builder stage
COPY --from=base /app/target/release/hammer /usr/local/bin/hammer

# Copy configuration file
COPY hammer/hammer.toml /app/hammer.toml

# Copy the resources directory for database migrations
COPY hammer/resources /app/resources

# Copy the actual JSON files from quarry directory (not symlinks)
COPY quarry/post_processed_abilities.json /app/abilities.json
COPY quarry/post_processed_items.json /app/items.json

# Copy entrypoint script
COPY hammer/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Create data directory and set permissions
RUN mkdir -p /app/data && chown -R appuser:appgroup /app

# Switch to non-root user
USER appuser

# Set working directory
WORKDIR /app

# Expose the port the app runs on
EXPOSE 8000

# Use entrypoint script
ENTRYPOINT ["/app/entrypoint.sh"]
