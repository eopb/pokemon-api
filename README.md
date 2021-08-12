# pokemon-api

## How to run

This project can be built with docker.

```bash
docker build -t pokemonapi .
docker run -it -p 8080:8080 pokemonapi bash
```

It is also possible to run with `cargo` if a [rust toolchain is installed](https://www.rust-lang.org/tools/install).

```bash
cargo run --release
```

