use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use awc::Client;
use pokeapi::PokemonOutput;
use serde::{Deserialize, Serialize};
mod pokeapi;

#[get("/pokemon/{pokemon}")]
async fn index(web::Path((pokemon,)): web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().json(PokemonOutput::get(&pokemon).await)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
