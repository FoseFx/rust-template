FROM rust:1.60.0 as builder

WORKDIR /usr/src/app
COPY . .
RUN cd bin && cargo install --path .


FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/bin /usr/local/bin/bin

CMD ["bin"]
