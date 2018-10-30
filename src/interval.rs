use eventual::Timer;

#[derive(Debug)]
pub struct Interval {
    pub timer: u32,
}

impl Interval {
    pub fn new<F>(timer_iter: u32, action: F)
    where
        F: Fn(),
    {
        let timer = Timer::new();
        for _ in timer.interval_ms(timer_iter).iter() {
            action()
        }
    }
}
