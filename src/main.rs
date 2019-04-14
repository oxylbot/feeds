#[macro_use]
extern crate serde;
extern crate hyper;
extern crate dotenv;
extern crate redis;
extern crate zmq;

mod services;
mod interval;

use dotenv::dotenv;
use redis::{Client, Connection};


fn main() {
    dotenv().ok();

}

#[cfg(test)]
mod tests {
    #[test]
    fn hello_world() {
        let hello: &str = "Hello, World!";
        assert_eq!(hello, "Hello, World!")
    }
}
