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

use std::thread;
use std::time::Duration;

fn main() {
    load_env().ok();

    handle_feed(String::from("youtube"), vec!["imstonic", "10"]);
}
