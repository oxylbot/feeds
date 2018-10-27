use super::feeders::Feeds;

use eventual::Timer;

pub fn start_interval(feeds: Feeds) {
    let timer = Timer::new();
    let ticks = timer.interval_ms(780 * 1000).iter();
    for _ in ticks {
        println!("{:?}", feeds);
    }
}
