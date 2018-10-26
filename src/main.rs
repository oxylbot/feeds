#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate reqwest;
extern crate futures;
extern crate serde_json;
extern crate tokio_timer;

mod feeders;
mod interval;

fn main() {
    println!("{:?}", feeders::return_feeds());
}
