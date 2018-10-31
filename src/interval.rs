use eventual::Timer;

use std::time::Duration;
use std::thread;

#[derive(Debug)]
pub struct Interval {
    break_it: bool,
    pause_dur: u64
}

impl Interval {
    pub fn new<F>(&self, timer_iter: u32, action: F) -> Self
    where
        F: Fn(),
    {
        let timer = Timer::new();

        for _ in timer.interval_ms(timer_iter).iter() {
            if self.break_it == true {
                break;
            } else {
                action();
                if self.pause_dur > 0 {
                    thread::sleep(Duration::from_secs(self.pause_dur));
                }
            }
        }

        Interval { 
            break_it: false,
            pause_dur: 0,
        }
    }

    pub fn pause_loop(&mut self, duration: u64) {
        self.pause_dur = duration;
    }

    pub fn break_loop(&mut self) {
        if self.break_it == true {
            println!("The loop is already broken!")
        } else {
            self.break_it = true
        }
    }
}
