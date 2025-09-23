use fugit::Instant;
use rtic_time::monotonic;
use stm32f4xx_hal::pac;

pub type Mono = stm32f4xx_hal::timer::MonoTimerUs<pac::TIM3>;
pub type TimestampUs = Instant<u64, 1, 1_000_000>;
