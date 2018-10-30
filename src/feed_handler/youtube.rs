use reqwest::get as fetch;
use serde_json::{from_str, Value};

use std::io::Read;

use std::env;

fn grab_results(url: &str) -> String {
    let mut content_data = String::new();

    fetch(url)
        .expect("Cannot grab data!")
        .read_to_string(&mut content_data)
        .expect("Cannot parse data to string!");

    content_data
}

pub fn grab_videos(channel: &str, limit: &str) -> Value {
    let request_string = format!("https://www.googleapis.com/youtube/v3/search?type=video&q={}&maxResults={}&part=snippet&key={}",
        channel,
        limit,
        env::var("YOUTUBE_API_KEY").expect("Cannot grab ENV variable"));

    let result: Value =
        from_str(&grab_results(request_string.as_str())).expect("Cannnot parse value");

    result
}
