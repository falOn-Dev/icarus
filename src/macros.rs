#[macro_export]
macro_rules! periodic_task {
    ($name:ident, $period:expr, $body:block) => {
        async fn $name(_: $name::Context) {
            let mut next = Mono::now() + $period;
            loop {
                $body
                crate::Mono::delay_until(next).await;
                next += $period;
            }
        }
        
    };
}