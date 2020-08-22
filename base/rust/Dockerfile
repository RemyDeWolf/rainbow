# inspired by https://shaneutt.com/blog/rust-fast-small-docker-image-builds/

# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/rainbow
COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

RUN rm -rf src/
RUN rm -f /usr/src/rainbow/target/x86_64-unknown-linux-musl/release/rainbow*
RUN rm -f /usr/src/rainbow/target/x86_64-unknown-linux-musl/release/deps/rainbow*
RUN rm -f /usr/src/rainbow/target/x86_64-unknown-linux-musl/release/rainbow.d

ADD src/main.rs src/main.rs
# copy the script to compute
ARG COMPUTE
ADD src/$COMPUTE.rs src/compute.rs

RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --release --target=x86_64-unknown-linux-musl

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:latest

RUN addgroup -g 1000 rainbow
RUN adduser -D -s /bin/sh -u 1000 -G rainbow rainbow

# directory for application files
RUN mkdir /data
RUN chown -R 1000 /data

WORKDIR /home/rainbow/bin/
COPY --from=cargo-build /usr/src/rainbow/target/x86_64-unknown-linux-musl/release/rainbow .
RUN chown rainbow:rainbow rainbow
USER rainbow
ARG COMPUTE
ENV COMPUTE=$COMPUTE
CMD ["./rainbow"]
