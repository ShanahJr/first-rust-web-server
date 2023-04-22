FROM rust:1.69 as builder

#This is creating a fresh cargo project so we can later cache the dependencies
RUN USER=root cargo new --bin rust_web_server
WORKDIR ./rust_web_server
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# This should cache the dependencies
RUN cargo build --release
# Remove all rust files from the source directory
RUN rm src/*.rs

COPY ./src ./src
COPY ./html ./html

# build for release
RUN rm ./target/release/deps/rust_web_server*
RUN cargo build --release

# Final base image
FROM debian:buster-slim

EXPOSE 8000

# Copy from the previous build
COPY --from=builder /rust_web_server/target/release/rust_web_server /usr/src/rust_web_server

# Run the binary
CMD ["/usr/src/rust_web_server"]