FROM rust:1.66.0
EXPOSE 8000
WORKDIR /build
RUN apt update && apt install lld clang -y
COPY . .

RUN cargo build --release

ENTRYPOINT [".target/release/zero2prod"]