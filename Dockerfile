FROM rust:1.54.0

EXPOSE 8080

WORKDIR /usr/src/pokemon-api
COPY . .

RUN cargo install --path .

ENTRYPOINT ["pokemon-api"]

