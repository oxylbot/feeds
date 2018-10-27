use eventual::Timer;

pub fn start_interval<F>(cb: F)
where
    F: Fn(),
{
    let timer = Timer::new();
    let ticks = timer.interval_ms(60000).iter();
    for _ in ticks {
        cb()
    }
}
