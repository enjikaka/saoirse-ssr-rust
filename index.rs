use http::{self, Request, Response, StatusCode};
use reqwest::{header, Client};
use serde_json::{Value};
use std::collections::HashMap;
use url::Url;

struct SaoirseResponse {
    id: String,
    name: String,
    artist: String,
    spotify_id: String,
    tidal_id: String,
    deezer_id: String,
    itunes_id: String,
    isrc_id: String,
}

fn handler(request: Request<()>) -> http::Result<Response<String>> {
    let uri_str = request.uri().to_string();
    let url = Url::parse(&uri_str).unwrap();
    let hash_query: HashMap<_, _> = url.query_pairs().to_owned().collect();

    match (hash_query.get("itemId"), hash_query.get("itemType"), hash_query.get("musicService")) {
        (Some(ref itemId), Some(ref itemType), Some(ref musicService)) => {
            let url = format!("https://api.saoir.se/{itemType}/{musicService}/{itemId}", itemType = itemType, musicService = musicService, itemId = itemId);
            let url = Url::parse(&url).expect("Failed to parse URL");
            let client = Client::new();
            let mut response = client
                .get(url)
                .header(header::ACCEPT, "text/html")
                .send()
                .expect("Failed to send HTTP request");

            assert_eq!(response.status(), StatusCode::OK);

            let rawText: Value = response.json().expect("Failed to get response HTML");

            let html_content = format!("<b>{name}</b>", name = rawText["name"]);

            let response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/html")
                .body(html_content)
                .expect("failed to render response");
            Ok(response)
        }

        _ => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("`selector` and `url` are required query params".to_string()),
    }
}