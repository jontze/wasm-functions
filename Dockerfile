FROM clux/muslrust:1.84.0-stable AS build_base
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
RUN cargo build --release --bin wasm-function-runtime --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static@sha256:3f2b64ef97bd285e36132c684e6b2ae8f2723293d09aae046196cca64251acac AS runtime
WORKDIR /runtime
COPY --from=builder --chown=nonroot:nonroot /runtime/target/x86_64-unknown-linux-musl/release/wasm-function-runtime wasm-function-runtime
USER nonroot
CMD [ "./wasm-function-runtime" ]
