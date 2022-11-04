FROM rust:1.65.0


COPY . .

RUN cargo build --release

EXPOSE 80

CMD ["./target/release/wh_server"]