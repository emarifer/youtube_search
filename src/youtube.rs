use gloo_net::http::Request;
use serde::Deserialize;

use crate::env::API_KEY;

pub async fn search_youtube(text_to_search: String) -> Result<VideoItem, gloo_net::Error> {
    // Llamar a la API de Youtube
    let query_url = format!(
        "https://www.googleapis.com/youtube/v3/search?part=id%2Csnippet&q={}&key={}",
        text_to_search, API_KEY
    );

    // let mut auth = String::from("Bearer ");
    // auth.push_str(API_KEY);

    let response = Request::get(&query_url)
        // .header("Authorization", &auth)
        .send()
        .await?;

    // web_sys::console::log_1(&text_to_search.into());

    let search_result = response.json::<SearchResult>().await?;

    let empty_video = build_empty_video();

    let video = match search_result.items.first() {
        Some(video) => {
            // let video_clone = video.clone();
            // web_sys::console::log_1(&video_clone.id.video_id.into());
            video
        }
        None => &empty_video,
    };

    Ok(video.clone())
}

fn build_empty_video() -> VideoItem {
    VideoItem {
        id: VideoItemId {
            kind: "".to_string(),
            video_id: "".to_string(),
        },
        snippet: VideoSnippet {
            title: "".to_string(),
            description: "".to_string(),
        },
    }
}

#[derive(Deserialize)]
struct SearchResult {
    // region_code: String,
    items: Vec<VideoItem>,
}

#[derive(Deserialize, Clone)]
pub struct VideoItem {
    pub id: VideoItemId,
    pub snippet: VideoSnippet,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoItemId {
    pub kind: String,
    pub video_id: String,
}

#[derive(Deserialize, Clone)]
pub struct VideoSnippet {
    pub title: String,
    pub description: String,
}
