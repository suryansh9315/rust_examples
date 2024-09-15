# FROM rust:1.58-alpine as builder     For Alpine
FROM rust:1.58 as builder

RUN cargo new --bin rust_examples
WORKDIR ./rust_examples
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

# FROM debian:buster-slim              For Debian
# FROM alpine:latest                   For Alpine
FROM centos:latest
COPY --from=builder /rust_examples/target/release/rust_examples ./rust_examples
CMD [ "./rust_examples" ]