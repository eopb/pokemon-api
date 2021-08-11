use serde::{Deserialize, Serialize};

pub async fn yoda(text: &str) -> Option<String> {
    translate(text, "yoda").await
}

pub async fn shakespear(text: &str) -> Option<String> {
    translate(text, "shakespear").await
}

#[derive(Deserialize)]
struct ApiResponse {
    content: Content,
}

#[derive(Deserialize)]
struct Content {
    translated: String,
}

async fn translate(text: &str, trans_type: &str) -> Option<String> {
    let request = serde_json::json!({
        "text": text,
    });

    let mut client = awc::Client::default();
    let response: ApiResponse = client
        .post(format!(
            "https://api.funtranslations.com/translate/{}.json",
            trans_type,
        ))
        .send_json(&request)
        .await
         //.ok()?
        .expect("1")
        .json()
        .await
        .expect("2");
         //.ok()?;
    Some(response.content.translated)
}
