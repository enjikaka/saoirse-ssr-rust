use http::{self, Request, Response, StatusCode};
use reqwest::{header, Client};
use serde_derive::{Serialize, Deserialize};
use serde_json;
use std::collections::HashMap;
use url::Url;
use handlebars::Handlebars;
// use std::fs;

#[derive(Serialize, Deserialize)]
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
    (Some(ref item_id), Some(ref item_type), Some(ref music_service)) => {
      let url = format!("https://api.saoir.se/{}/{}/{}", item_type, music_service, item_id);
      let url = Url::parse(&url).expect("Failed to parse URL");
      let client = Client::new();
      let mut response = client
        .get(url)
        .header(header::ACCEPT, "text/html")
        .send()
        .expect("Failed to send HTTP request");

      assert_eq!(response.status(), StatusCode::OK);
      let text = response.text().expect("Failed ot parse text");


      let data: SaoirseResponse = serde_json::from_str(&text).unwrap();

      let handlebars = Handlebars::new();
      // TODO: @now/rust can't read files after deployment
      // let template_from_file = fs::read_to_string("song.hbs").expect("Could not read file");
      // HACK: Read via rawgit from network instead.

      let template_from_rawgit = reqwest::get("https://cdn.rawgit.com/enjikaka/saoirse-ssr-rust/master/song.hbs")
        .expect("Could not fetch HBS file from Rawgit")
        .text()
        .expect("Could not parse file as text");

      let html_content = handlebars.render_template(&template_from_rawgit, &data).expect("Failed to render template");

      let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html")
        .body(html_content)
        .expect("failed to render response");

      Ok(response)
    }

    _ => Response::builder()
      .status(StatusCode::BAD_REQUEST)
      .body("`itemId`, `itemType` and `musicService` are required query params".to_string()),
  }
}

/* For testing locally, uncomment and run: cargo run
fn main() {
  let mut request = Request::builder();

  request.uri("http://localhost?musicService=tidal&itemType=track&itemId=105114863");

  let response = handler(request.body(()).unwrap()).expect("Request got handled");

  println!("{:?}", response);
}
*/
