
//! blinky Sample
//!
//! pi8 is used as art-pi LED.
//!
#![deny(warnings)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(100.MHz()).freeze(vos, &dp.SYSCFG);

    // Acquire the GPIOA peripheral. This also enables the clock for
    // let gpioa = dp.GPIOA.split(&mut ccdr.ahb4);
    // Acquire the GPIOI peripheral. This also enables the clock for
    let gpioi = dp.GPIOI.split(ccdr.peripheral.GPIOI);

    // Configure gpio I pin 8 as a push-pull output.
    let mut led = gpioi.pi8.into_push_pull_output();
    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
            led.set_high();
            delay.delay_ms(100_u8);

            led.set_low();
            delay.delay_ms(100_u8);
    }
}
