FROM rust:1.70

WORKDIR /usr/src/hello
COPY . .

RUN cargo install --path .

CMD ["hello"]
