FROM rust:latest

RUN cargo install cargo-make

WORKDIR /app
COPY . .

RUN cargo install --path .

CMD ["cargo", "make", "release"]
