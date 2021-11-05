FROM rust:1.56.1-slim

# Tooling setup
RUN rustup toolchain install nightly
RUN rustup component add llvm-tools-preview
RUN cargo install cargo-fuzz
RUN cargo install cargo-binutils

# Src import
RUN mkdir /scapegoat
WORKDIR /scapegoat
COPY . /scapegoat/

# Build
RUN export SG_MAX_STACK_ELEMS=1024
RUN cargo build