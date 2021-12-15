# MSRV
FROM rust:1.55-slim

# Non-Rust tooling
ENV TZ=US/New_York
RUN apt-get update -y
RUN DEBIAN_FRONTEND="noninteractive" apt-get install -y \
    rr \
    tree \
    vim \
    musl-tools

# Rust tooling
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup component add rust-src --toolchain nightly
RUN rustup component add llvm-tools-preview
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo install cargo-fuzz
RUN cargo install cargo-binutils
RUN cargo install cargo-bloat

# Src import
RUN mkdir /scapegoat
WORKDIR /scapegoat
COPY . /scapegoat/

# MSRV Test
RUN cargo test