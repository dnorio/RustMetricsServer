# Development Stage
FROM rust:1.82.0-bookworm as dev
WORKDIR /app
RUN cargo install cargo-watch
COPY . .
RUN cargo build --release

# Production Stage
FROM debian:bookworm-slim as prod
WORKDIR /app
COPY --from=dev /app/target/release/my_rust_server .
CMD ["./my_rust_server"]