#[macro_export]
macro_rules! periodic_task {
    ($name:ident, $period:expr, $body:block) => {
        let period: <Mono as rtic_time::Monotonic>::Duration = ($period).into();

        let mut next = Mono::now() + period;
        loop {
            $body
            // Check for overrun
            if Mono::now() > next {
                let overrun = Mono::now() - next;
                defmt::warn!(
                    "{} overran period of {} ms by {} ms",
                    stringify!($name),
                    period.to_millis(),
                    overrun.to_millis()
                );
                next = Mono::now();
            }
            Mono::delay_until(next).await;
            next += $period;
        }
        
    };
}