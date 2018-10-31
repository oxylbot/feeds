mod reddit;
mod twitch;
mod youtube;

use std::convert::AsRef;

pub fn load_feed(feed: String, arguments: Vec<&str>) {
    match feed.as_ref() {
        "reddit" => {
            if arguments.len() > 2 || arguments.len() < 2 {
                println!("Inufficent arugments!");
                return;
            } else {
                let reddit_feed = reddit::grab_subreddit(arguments[0], arguments[1]);
                println!("{:?}", reddit_feed);
            }
        }
        "youtube" => {
            if arguments.len() > 2 || arguments.len() < 2 {
                println!("Inufficent arugments!");
                return;
            } else {
                let youtube_feed = youtube::grab_videos(arguments[0], arguments[1]);
                println!("{:?}", youtube_feed);
            }
        }
        "twitch" => {
            if arguments.is_empty() {
                println!("We at least need one channel");
            } else {
                let twitch_feed = twitch::grab_streamers(arguments);
                println!("{:?}", twitch_feed);
            }
        }
        "twitter" => println!("Do twitter something"),
        _ => println!("Unknown feed"),
    }
}
