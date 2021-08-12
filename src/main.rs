mod pokeapi;
mod translate;

use pokeapi::Pokemon;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use tracing::{info, Level};
use tracing_actix_web::TracingLogger;
use tracing_subscriber::fmt::format::FmtSpan;

static SOCKET_ADD: &str = "127.0.0.1:8080";

#[get("/pokemon/{pokemon}")]
async fn pokemon_info(web::Path((pokemon,)): web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().json(Pokemon::get(&pokemon).await)
}

#[get("/pokemon/translated/{pokemon}")]
async fn translated(web::Path((pokemon,)): web::Path<(String,)>) -> impl Responder {
    let mut pokemon = Pokemon::get(&pokemon).await.unwrap();

    pokemon.description = if pokemon.is_legendary || &pokemon.habitat == "cave" {
        translate::yoda(&pokemon.description).await
    } else {
        translate::shakespeare(&pokemon.description).await
    }
    // if anything fails use the standard description
    .unwrap_or(pokemon.description);

    HttpResponse::Ok().json(pokemon)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // set subscriber to print tracing logs to stdout
    {
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(Level::INFO)
            .with_span_events(FmtSpan::CLOSE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("no global subscriber has been set");
    }

    info!("Starting server");

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger)
            .service(pokemon_info)
            .service(translated)
    })
    .bind(SOCKET_ADD)?
    .run()
    .await
}
