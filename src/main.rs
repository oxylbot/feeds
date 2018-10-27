#[macro_use]
extern crate serde_derive;

extern crate dotenv;
extern crate eventual;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

mod feeders;
mod interval;

use dotenv::dotenv as load_env;

fn main() {
    load_env().ok();

    interval::start_interval(move || {
        let feeds = feeders::return_feeds();
        println!("{:?}", feeds);
    });
}
