FROM rustlang/rust:nightly as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:12.5-slim

WORKDIR /app
RUN ["apt","update"]
RUN ["apt","install","-y","ca-certificates"]

ENV API_KEY=

COPY --from=builder /app/target/release/teonite .
COPY --from=builder /app/.sample.env ./.env
COPY --from=builder /app/README.md README.md