FROM rust:1.54.0

WORKDIR /usr/src/pokemon-api
COPY . .

RUN cargo build --release
# RUN cargo install --path .

# ENTRYPOINT ["pokemon-api"]

