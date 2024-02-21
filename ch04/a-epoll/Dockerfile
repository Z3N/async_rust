ARG RUST_VERSION=1.76.0
ARG APP_NAME=a-epoll

FROM rust:${RUST_VERSION}-bookworm as cheff
RUN cargo install cargo-chef
WORKDIR /app

FROM cheff as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
FROM cheff as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path /app/recipe.json
COPY . .
ARG APP_NAME
RUN cargo build

CMD ["/app/target/debug/a-epoll"]