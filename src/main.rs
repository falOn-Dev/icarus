#![no_main]
#![no_std]
#![allow(warnings)]
#![allow(unused_variables)]

mod types;
mod macros;

use defmt_rtt as _;
use panic_probe as _;
use rtic::app;

use nalgebra as na;
use rtic_time::Monotonic;
use stm32f4xx_hal::{pac, prelude::*, timer::DMAR};

pub use types::Mono;

#[app(device = stm32f4xx_hal::pac)]
mod app {

    use core::task;

    use cortex_m::singleton;
    use stm32f4xx_hal::{
        dma::{config, MemoryToPeripheral, Stream1, StreamsTuple, Transfer}, gpio, pac::{DMA2, TIM1}, rcc::Config, timer::PwmChannel
    };

    use crate::types::TimestampUs;

    use super::*;

    #[shared]
    struct SharedResources {}

    #[local]
    struct LocalResources {}

    #[init]
    fn init(mut ctx: init::Context) -> (SharedResources, LocalResources) {
        let dp = ctx.device;

        let clock_cfg = Config::hse(8.MHz())
            .sysclk(168.MHz())
            .hclk(168.MHz());

        let mut rcc = dp.RCC.freeze(clock_cfg);

        dp.TIM3.monotonic_us(&mut ctx.core.NVIC, &mut rcc);

        defmt::info!("Initialization complete");

        every_second::spawn().unwrap();
        every_five_seconds::spawn().unwrap();
        overrun_bait::spawn().unwrap();

        (SharedResources {}, LocalResources {})
    }

    // Every 1 Second
    #[task]
    async fn every_second(ctx: every_second::Context) {
        periodic_task!(every_second, 1.secs(), {
            defmt::info!("1 second passed");
        });
    }

    // Every 5 seconds
    #[task]
    async fn every_five_seconds(ctx: every_five_seconds::Context) {
        periodic_task!(every_five_seconds, 5.secs(), {
            defmt::info!("5 seconds passed");
        });
    }

    #[task]
    async fn overrun_bait(ctx: overrun_bait::Context) {
        periodic_task!(overrun_bait, 3.secs(), {
            defmt::info!("Starting long task");
            // Simulate a long task that causes an overrun
            Mono::delay(4000.millis().into()).await;
            defmt::info!("Long task complete");
        });
    }

}
