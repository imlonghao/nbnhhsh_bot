FROM rust:1.82 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/nbnhhsh_bot /usr/local/bin/nbnhhsh_bot
CMD ["nbnhhsh_bot"]
