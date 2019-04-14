use hyper::rt::{self, Stream, Future};
use hyper::{self, Client, Request};

use std::io;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct YoutubeData {
    pub video_id: String,
    pub channel_id: String,
    pub video_title: String,
    pub channel_title: String
}

pub fn return_data() {
    let api_key = env::var("YOUTUBE_KEY").unwrap_or_else(|_| panic!("set \"YOUTUBE_KEY\" to your environment"));
}
