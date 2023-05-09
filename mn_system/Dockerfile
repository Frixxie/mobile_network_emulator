FROM rust:latest

WORKDIR /usr/src/mn_system
COPY . .

RUN cargo install --path .

CMD "mn_system" -h 0.0.0.0 -p 8080 -d mongodb://root:example@mongo:27017/
