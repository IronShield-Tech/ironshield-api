# Stage 1: Build with a simple caching strategy
FROM rust:bullseye AS builder
WORKDIR /app

# Copy the dependency manifest
COPY Cargo.toml ./

# Fetch all dependencies to cache them
RUN cargo fetch

# Copy the rest of your application's source code
COPY . .

# Build the application. This will be faster since dependencies are already downloaded.
RUN cargo build --release

# Stage 2: Create the final, small image
FROM debian:bullseye-slim
COPY --from=builder /app/target/release/ironshield-api /usr/local/bin/
EXPOSE 3000
CMD ["ironshield-api"]