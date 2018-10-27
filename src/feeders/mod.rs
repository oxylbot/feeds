pub mod reddit;
pub mod twitch;

use self::reddit::RedditFeedConstructor;
use self::twitch::{StreamFeedConstr, TwitchFeedConstructor};

use serde_json::Value;

use std::env;

#[derive(Debug)]
pub struct Feeds {
    pub reddit: Value,
    pub twitch: StreamFeedConstr,
}

fn grab_reddit_feeds() -> Value {
    let feed_constructor = RedditFeedConstructor::new("all", "new");
    feed_constructor.return_results()
}

fn grab_twitch_feeds() -> StreamFeedConstr {
    let feed_constructor = TwitchFeedConstructor::new(
        env::var("TWITCH_CLIENT_ID").unwrap(),
        vec!["riotgames".to_string(), "cowsep".to_string()],
    );
    feed_constructor.return_results()
}

pub fn return_feeds() -> Feeds {
    Feeds {
        reddit: grab_reddit_feeds(),
        twitch: grab_twitch_feeds(),
    }
}
