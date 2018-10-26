pub mod reddit;

use self::reddit::RedditFeedConstructor;

use serde_json::Value;

#[derive(Debug)]
pub struct Feeds {
    pub reddit: Value,
}

fn grab_reddit_feeds() -> Value {
    let feed_constructor = RedditFeedConstructor::new("all", "new");
    feed_constructor.return_results()
}

pub fn return_feeds() -> Feeds {
    Feeds { reddit: grab_reddit_feeds() }
}
