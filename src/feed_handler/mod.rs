mod reddit;
mod youtube;

use super::interval::*;

use std::convert::AsRef;

pub fn handle_feed(feed: String, arguments: Vec<&str>) {
    match feed.as_ref() {
        "reddit" => Interval::new(60000 * 3, || {
            if arguments.len() > 2 {
                panic!("Too many arguments found!")
            } else {
                let reddit_feed = reddit::grab_subreddit(arguments[0], arguments[1]);
                println!("{:?}", reddit_feed);
            }
        }),
        "youtube" => Interval::new(60000, || {
            if arguments.len() > 2 || arguments.len() < 2 {
                panic!("Inufficent arugments!")
            } else {
                let youtube_feed = youtube::grab_videos(arguments[0], arguments[1]);
                println!("{:?}", youtube_feed);
            }
        }),
        _ => println!("Unknown feed"),
    }
}
