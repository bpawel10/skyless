FROM rust:alpine AS build

RUN apk -U upgrade
RUN apk add --no-cache musl-dev openssl-dev

WORKDIR /skyless
COPY . .

ADD cargo.toml Cargo.toml
ADD cargo.lock Cargo.lock
ADD core/cargo.toml core/Cargo.toml
ADD core/cargo.lock core/Cargo.lock
ADD macros/cargo.toml macros/Cargo.toml
ADD macros/cargo.toml macros/Cargo.lock

ENV RUSTFLAGS=-Ctarget-feature=-crt-static
RUN cargo build --release --locked


FROM alpine:latest

RUN apk -U upgrade
RUN apk add --no-cache libgcc openssl

WORKDIR /skyless
COPY --from=build /skyless/target/release/skyless_core ./skyless

EXPOSE 7171
EXPOSE 7172

CMD ["./skyless"]
