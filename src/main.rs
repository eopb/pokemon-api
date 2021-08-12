use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use awc::Client;
use pokeapi::PokemonOutput;
use serde::{Deserialize, Serialize};
use tracing::{debug, Level};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::format::FmtSpan;

mod pokeapi;
mod translate;

#[get("/pokemon/{pokemon}")]
async fn index(web::Path((pokemon,)): web::Path<(String,)>) -> impl Responder {
    HttpResponse::Ok().json(PokemonOutput::get(&pokemon).await)
}

#[get("/pokemon/translated/{pokemon}")]
async fn translated(web::Path((pokemon,)): web::Path<(String,)>) -> impl Responder {
    let mut pokemon = PokemonOutput::get(&pokemon).await.unwrap();

    pokemon.description = if pokemon.is_legendary || &pokemon.habitat == "cave" {
        translate::yoda(&pokemon.description).await
    } else {
        translate::shakespeare(&pokemon.description).await
    }
    .unwrap_or(pokemon.description);

    HttpResponse::Ok().json(pokemon)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    debug!("hi");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger)
            .service(index)
            .service(translated)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
