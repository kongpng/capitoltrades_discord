# Stage 1: Build the binaries
FROM rust:1.79 as builder

# Install SQLx CLI for database management
RUN cargo install sqlx-cli

# Set the working directory for the build context
WORKDIR /workspace

# Copy the dependency manifest files
COPY crates/telegram_bot/Cargo.toml crates/telegram_bot/Cargo.toml
COPY crates/capitoltrades_api/Cargo.toml crates/capitoltrades_api/Cargo.toml
COPY Cargo.lock Cargo.lock

# Copy the source tree and build the project
COPY crates crates

# Build the project to fetch the dependencies
WORKDIR /workspace/crates/telegram_bot
RUN cargo build --release

# Stage 2: Create the runtime image
FROM debian:buster-slim

# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the binaries from the builder stage
COPY --from=builder /workspace/crates/telegram_bot/target/release/bot /app/bot
COPY --from=builder /workspace/crates/telegram_bot/target/release/notify /app/notify

# Copy the migration files
COPY crates/telegram_bot/migrations ./migrations

# Set environment variables (update these as necessary)
ENV DATABASE_URL=postgres://user:password@db:5432/capitoltrades_db
ENV DISCORD_TOKEN=your_discord_token_here

# Expose necessary ports (if any)
EXPOSE 8080

