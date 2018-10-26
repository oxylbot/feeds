use reqwest::get as fetch;
use serde_json::{from_str, Value};

use std::io::Read;

pub struct RedditFeedConstructor {
    pub subreddit: &'static str,
    pub feed_type: &'static str,
}

fn grab_feeds(subreddit: &'static str, feed_type: &'static str) -> String {
    let mut content_data = String::new();

    let request_string = format!("https://www.reddit.com/r/{}/{}.json", subreddit, feed_type);

    fetch(request_string.as_str())
        .expect("Cannot grab feeds")
        .read_to_string(&mut content_data)
        .expect("Cannot return feeds as a string!");

    content_data
}

impl RedditFeedConstructor {
    pub fn new(subreddit: &'static str, feed_type: &'static str) -> Self {
        RedditFeedConstructor {
            subreddit,
            feed_type,
        }
    }

    pub fn return_results(&self) -> Value {
        let results: Value = from_str(grab_feeds(self.subreddit, self.feed_type).as_str()).unwrap();
        results
    }
}
