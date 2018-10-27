use reqwest::get as fetch;
use serde_json::from_str;

use std::io::Read;

pub struct TwitchFeedConstructor {
    client_id: String,
    channel_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamConstr {
    pub _id: usize,
    pub average_fps: f32,
    pub channel: ChannelConstr,
    pub created_at: String,
    pub delay: i32,
    pub game: String,
    pub is_playlist: bool,
    pub preview: PreviewConstr,
    pub video_height: i32,
    pub viewers: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PreviewConstr {
    pub large: String,
    pub medium: String,
    pub small: String,
    pub template: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelConstr {
    pub _id: usize,
    pub broadcaster_language: String,
    pub created_at: String,
    pub display_name: String,
    pub followers: i32,
    pub game: String,
    pub language: String,
    pub logo: String,
    pub mature: bool,
    pub name: String,
    pub partner: bool,
    pub profile_banner: String,
    pub profile_banner_background_color: Option<String>,
    pub status: String,
    pub updated_at: String,
    pub url: String,
    pub video_banner: String,
    pub views: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StreamFeedConstr {
    pub _total: i32,
    pub streams: Vec<StreamConstr>
}

impl TwitchFeedConstructor {
    pub fn new(client_id: String, channel_ids: Vec<String>) -> Self {
        TwitchFeedConstructor {
            client_id,
            channel_ids,
        }
    }

    fn grab_feeds(&self) -> String {
        let mut content_data = String::new();

        let request_string = format!(
            "https://api.twitch.tv/kraken/streams/?channel={}&client_id={}",
            self.channel_ids.join(","),
            self.client_id
        );

        println!("{}", request_string);

        fetch(request_string.as_str())
            .expect("Cannot grab streams")
            .read_to_string(&mut content_data)
            .expect("Cannot return results as a string!");

        content_data
    }

    pub fn return_results(&self) -> StreamFeedConstr {
        let results: StreamFeedConstr = from_str(self.grab_feeds().as_str()).unwrap();

        results
    }
}
