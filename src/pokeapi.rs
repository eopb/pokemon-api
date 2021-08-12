use serde::{Deserialize, Serialize};
use tracing::{error, instrument};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Pokemon {
    name: String,
    pub description: String,
    pub habitat: String,
    #[serde(rename = "isLegendary")]
    pub is_legendary: bool,
}

impl Pokemon {
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

/// The shape of data returned by pokeapi
#[derive(Deserialize)]
struct PokeApiData {
    name: String,
    is_legendary: bool,
    habitat: Habitat,
    flavor_text_entries: Vec<FlavorText>,
}

#[derive(Deserialize)]
struct FlavorText {
    flavor_text: String,
    language: Language,
}

#[derive(Deserialize)]
struct Language {
    name: String,
}

#[derive(Deserialize)]
struct Habitat {
    name: String,
}

impl PokeApiData {
    #[instrument(level = "info")]
    async fn get(pokemon: &str) -> Option<Self> {
        let client = awc::Client::default();
        client
            .get(format!(
                "https://pokeapi.co/api/v2/pokemon-species/{}",
                pokemon
            ))
            .send()
            .await
            .map_err(|e| error!("Failed to connect to pokeapi: {:?}", e))
            .ok()?
            .json()
            .await
            .map_err(|e| error!("Failed to parse json from pokeapi: {:?}", e))
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[actix_rt::test]
    async fn standard() {
        assert_eq!(
            Pokemon::get("ditto").await.unwrap(),
            Pokemon {
                name: "ditto".to_owned(),
                description: "It can freely recombine its own cellular structure to\ntransform into other life-forms.".to_owned(),
                habitat: "urban".to_owned(),
                is_legendary: false,
            }
        );
    }
    #[actix_rt::test]
    async fn legendary() {
        assert_eq!(
            Pokemon::get("mewtwo").await.unwrap(),
            Pokemon {
                name: "mewtwo".to_owned(),
                description: "It was created by\na scientist after\nyears of horrific\u{c}gene splicing and\nDNA engineering\nexperiments.".to_owned(),
                habitat: "rare".to_owned(),
                is_legendary: true,
            }
        );
    }
}
