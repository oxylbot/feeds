use reqwest::get as fetch;
use serde_json::{from_str, Value};

use std::io::Read;
use std::env;

fn grab_results(url: &str) -> String {
    let mut content_data = String::new();

    fetch(url)
        .expect("Cannot grab body from this URL.")
        .read_to_string(&mut content_data)
        .expect("Cannot parse data to this string!");

    content_data
}

pub fn grab_streamers(channels: Vec<&str>) -> Value {
    let request_string = format!(
        "https://api.twitch.tv/kraken/streams/?channel={}&client_id={}",
        channels.join(","),
        env::var("TWITCH_CLIENT_ID").unwrap()
    );

    let result: Value = from_str(&grab_results(request_string.as_str())).unwrap();

    result
}
