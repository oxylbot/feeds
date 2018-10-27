#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate eventual;

mod feeders;
mod interval;

fn main() {
    interval::start_interval(move || {
        let feeds = feeders::return_feeds();
        println!("{:?}", feeds);
    });
}
