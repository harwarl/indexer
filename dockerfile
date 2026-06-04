FROM rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/server .
EXPOSE 3000
CMD [ "indexer" ]
