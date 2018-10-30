use reqwest::get as fetch;
use serde_json::{from_str, Value};

use std::io::Read;

fn grab_results(url: &str) -> String {
    let mut content_data = String::new();

    fetch(url)
        .expect("Cannot grab body from this URL.")
        .read_to_string(&mut content_data)
        .expect("Cannot parse to string!");

    content_data
}

pub fn grab_subreddit(subreddit: &str, sub_type: &str) -> Value {
    let request_string = format!("https://www.reddit.com/r/{}/{}.json", subreddit, sub_type);

    let result: Value = from_str(&grab_results(request_string.as_str())).unwrap();

    result
}
