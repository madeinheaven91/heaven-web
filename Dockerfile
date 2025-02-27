FROM rust:1.85-slim

WORKDIR /usr/src/heaven-web
COPY . .

CMD ["cargo", "run"]
