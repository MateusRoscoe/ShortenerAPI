# Rust as the base image
FROM rust:1.78 as build

# Create a new empty shell project
RUN USER=root cargo new --bin shortener_api
WORKDIR /shortener_api

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release
RUN rm ./target/release/deps/shortener_api*
RUN cargo build --release

# The final base image
FROM debian:bookworm

# Copy from the previous build
COPY --from=build /shortener_api/target/release/shortener_api /usr/src/shortener_api

# Run the binary
CMD ["/usr/src/shortener_api"]