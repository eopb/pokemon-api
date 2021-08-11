use actix_web::{get, web, App, HttpServer, Responder};
use awc::Client;
use serde::{Deserialize, Serialize};
use pokeapi::PokemonOutput;
mod pokeapi;


#[get("/pokemon/{pokemon}")]
async fn index(web::Path((pokemon,)): web::Path<(String,)>) -> impl Responder {
    dbg!(PokemonOutput::get("ditto").await);
    "hi"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
