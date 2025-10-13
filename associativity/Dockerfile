FROM rust:1.90.0-alpine3.22 as build
WORKDIR /app
COPY src ./src
COPY Cargo.toml .
RUN cargo build --release

FROM alpine:3.22
WORKDIR /app
COPY --from=build /app/target/release/test .
CMD ["./test"]