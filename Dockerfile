FROM rust:1.56 as cargo-chef-rust
RUN cargo install cargo-chef
RUN rustup component add rustfmt

FROM cargo-chef-rust as planner
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM cargo-chef-rust as cacher
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM cargo-chef-rust as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin steward

FROM cargo-chef-rust as runtime
WORKDIR /app
COPY --from=builder /app/target/release/steward /usr/local/bin
CMD steward cosmos-signer
