use actix_web::{get, web, App, HttpServer, Responder};
use awc::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct PokemonOutput {
    name: String,
    description: String,
    habitat: String,
    #[serde(rename = "isLegendary")]
    is_legendary: bool,
}

impl PokemonOutput {
    pub async fn get(pokemon: &str) -> Option<Self> {
        let PokeApiData {
            name,
            is_legendary,
            habitat: Habitat { name: habitat },
            flavor_text_entries,
        } = PokeApiData::get(pokemon).await?;
        Some(Self {
            name,
            habitat,
            is_legendary,
            description: flavor_text_entries
                .into_iter()
                .find(|f| &f.language.name == "en")?
                .flavor_text,
        })
    }
}

#[derive(Deserialize, Debug)]
struct PokeApiData {
    name: String,
    is_legendary: bool,
    habitat: Habitat,
    flavor_text_entries: Vec<FlavorText>,
}

#[derive(Deserialize, Debug)]
struct FlavorText {
    flavor_text: String,
    language: Language,
}

#[derive(Deserialize, Debug)]
struct Language {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Habitat {
    name: String,
}

impl PokeApiData {
    async fn get(pokemon: &str) -> Option<Self> {
        let mut client = awc::Client::default();
        client
            .get(format!(
                "https://pokeapi.co/api/v2/pokemon-species/{}",
                pokemon
            )) // <- Create request builder
            .send() // <- Send http request
            .await
            .ok()?
            .json()
            .await
            .ok()
    }
}
