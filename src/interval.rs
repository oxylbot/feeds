use tokio_timer::{Timer};
use futures::stream::Stream;

use std::time::Duration;

pub fn start_interval(cb: &Fn()) {
    Timer::default()
    // Interval::new_interval(Duration::new(60 * 13, 0))
    //     .map_err(|e| println!("Error occurd in inerval : {}", e))
    //     .take_while(|()| cb());
}
