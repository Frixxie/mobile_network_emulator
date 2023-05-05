FROM rust:latest

WORKDIR /usr/src/mn_system
COPY . .

RUN cargo install --path .

CMD ["mn_system"]
