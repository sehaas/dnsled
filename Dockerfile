FROM rust:1-bookworm AS builder
LABEL builder=true

# copy code files
COPY Cargo.toml Cargo.lock /code/
COPY /src/ /code/src/

# build code
WORKDIR /code
RUN cargo build --release

# runtime container
FROM debian:bookworm AS runtime

RUN apt-get update && apt-get install -y \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# set default logging, can be overridden
ENV RUST_LOG=info

# copy binary
COPY --from=builder /code/target/release/dnsled /usr/local/bin/dnsled

EXPOSE 53/udp

# set entrypoint
ENTRYPOINT ["/usr/local/bin/dnsled"]
