use std::{collections::HashMap, sync::Mutex};

use serde::{Deserialize, Serialize};
use tracing::{info, instrument, warn};
use once_cell::sync::Lazy;


// funtranslations is very rate limited and slow so I've opted to use an in memory cache.
type Cache = Lazy<Mutex<HashMap<String, String>>>;
static YODA_CACHE: Cache = Lazy::new(|| Mutex::new(HashMap::new()));
static SHAKESPEARE_CACHE: Cache = Lazy::new(|| Mutex::new(HashMap::new()));

#[instrument(level = "info")]
pub async fn yoda(text: &str) -> Option<String> {
    translate(text, "yoda", &YODA_CACHE).await
}

#[instrument(level = "info")]
pub async fn shakespeare(text: &str) -> Option<String> {
    translate(text, "shakespeare", &SHAKESPEARE_CACHE).await
}

async fn translate(text: &str, trans_type: &str, cache: &Cache) -> Option<String> {
    if let Ok(mut cache) = cache.lock() {
        if let Some(translation) = cache.get(text) {
            info!("Taking translation from cache");
            return Some(translation.to_owned());
        }
        let translation = funtranslations(text, trans_type).await;
        if let Some(ref translation) = translation {
            cache.insert(text.to_owned(), translation.to_owned());
        };
        translation
    } else {
        warn!("Unable to get lock on cache");
        funtranslations(text, trans_type).await
    }
}

async fn funtranslations(text: &str, trans_type: &str) -> Option<String> {
    #[derive(Serialize)]
    struct PostBody<'a> {
        text: &'a str,
    }

    #[derive(Deserialize)]
    struct Response {
        contents: Contents,
    }
    #[derive(Deserialize)]
    struct Contents {
        translated: String,
    }

    let client = awc::Client::default();
    let response: Response = client
        .post(format!(
            "https://api.funtranslations.com/translate/{}.json",
            trans_type,
        ))
        .send_form(&PostBody { text })
        .await
        .map_err(|e| warn!("Failed to connect to funtranslations: {:?}", e))
        .ok()?
        .json()
        .await
        .map_err(|e| warn!("failed to parse json from funtranslations: {:?}", e))
        .ok()?;
    Some(response.contents.translated)
}
