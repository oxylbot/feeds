#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate eventual;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod feed_handler;
mod interval;

use self::feed_handler::*;
use dotenv::dotenv as load_env;

fn main() {
    load_env().ok();

    load_feed(String::from("twitch"), vec!["sypherpk", "chicalive"]);
}
