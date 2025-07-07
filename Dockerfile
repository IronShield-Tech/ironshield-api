# Stage 1: Build the application using a specific Debian version
FROM rust:bullseye AS builder
WORKDIR /app

# Copy the entire API package into the container
COPY . .

# Build the application.
RUN cargo build --release

# Stage 2: Create the final, small image.
FROM debian:bullseye-slim
COPY --from=builder /app/target/release/ironshield-api /usr/local/bin/
EXPOSE 3000
CMD ["ironshield-api"]