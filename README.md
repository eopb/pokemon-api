# pokemon-api
Interview challenge.

If I was doing this in production, I would provide a better response for when pokeapi fails to return anything.
Such a response would be a little json explaining that the request had failed.

I would have also renamed and squashed a few commits to get a cleaner git history.
You asked for the git history so I didn't mess with it this time.


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

