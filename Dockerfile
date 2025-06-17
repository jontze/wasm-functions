FROM clux/muslrust:1.87.0-stable AS build_base
USER root
RUN cargo install cargo-chef
WORKDIR /runtime

FROM build_base AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin wasm-function-runtime

FROM build_base AS cacher
COPY --from=planner /runtime/recipe.json recipe.json
ENV RUSTUP_MAX_RETRIES=100
ENV CARGO_INCREMENTAL=0
ENV CARGO_NET_RETRY=100
ENV CARGO_TERM_COLOR=always
# Build dependencies - this is the dependencies caching layer
RUN cargo chef cook --release --recipe-path recipe.json --bin wasm-function-runtime --target x86_64-unknown-linux-musl

FROM build_base AS builder
COPY . .
COPY --from=cacher /runtime/target target
COPY --from=cacher /root/.cargo /root/.cargo
ENV RUSTUP_MAX_RETRIES=100
ENV CARGO_INCREMENTAL=0
ENV CARGO_NET_RETRY=100
ENV CARGO_TERM_COLOR=always
# Build and cache only the app with the previously built dependencies
RUN cargo build --release --bin wasm-function-runtime --target x86_64-unknown-linux-musl && mkdir -p /runtime/data/db && mkdir -p /runtime/data/functions

FROM gcr.io/distroless/static@sha256:b7b9a6953e7bed6baaf37329331051d7bdc1b99c885f6dbeb72d75b1baad54f9 AS runtime
WORKDIR /runtime
COPY --from=builder --chown=nonroot:nonroot --chmod=700 /runtime/data data
COPY --from=builder --chown=nonroot:nonroot /runtime/target/x86_64-unknown-linux-musl/release/wasm-function-runtime wasm-function-runtime
USER nonroot
CMD [ "./wasm-function-runtime" ]
