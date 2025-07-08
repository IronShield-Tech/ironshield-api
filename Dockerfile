# Stage 1: Build with dependency caching
FROM rust:bullseye AS builder
WORKDIR /app

# Initialize a dummy project to cache dependencies against.
RUN USER=root cargo init

# We need the lib file for the dummy build step
RUN touch src/lib.rs

# Copy the dependency manifest
COPY Cargo.toml ./

# Build and cache the dependencies. This is the slow part that
# will only run again if Cargo.toml changes.
RUN cargo build --release

# Now copy your actual application source code.
COPY . .

# Build your application. This will be very fast because
# dependencies are cached and it only rebuilds your code.
RUN cargo build --release

# Stage 2: Create the final, small image
FROM debian:bullseye-slim
COPY --from=builder /app/target/release/ironshield-api /usr/local/bin/
EXPOSE 3000
CMD ["ironshield-api"]