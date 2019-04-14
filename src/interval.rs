use std::thread;
use std::time::Duration;

pub struct Interval {
    break_loop: bool,
    interval: u64
}

impl Interval {
    pub fn new() -> Self {
        Interval {
            break_loop: false,
            interval: 4500
        }
    }

    pub fn start<F>(&self, f: F)
    where F: Fn() {
        while self.break_loop != true {
            f();
            thread::sleep(Duration::from_millis(self.interval));
        }
    }

    pub fn r#break(&mut self) {
        self.break_loop = true;
    }
}
