FROM rust as planner
WORKDIR /build
# We only pay the installation cost once,
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef
COPY ./ ./
RUN cargo chef prepare --recipe-path recipe.json

FROM rust as cacher
WORKDIR /build
RUN cargo install cargo-chef
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR /build
COPY ./ ./
# Copy over the cached dependencies
COPY --from=cacher /build/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release

FROM rust as runtime
WORKDIR /build
ENV ROCKET_ADDRESS 0.0.0.0
EXPOSE 8000
COPY --from=builder /build/target/release/server /usr/local/bin
COPY db/migrations/ ./migrations
ENTRYPOINT ["/usr/local/bin/server"]
