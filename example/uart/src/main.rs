
//! Echo bytes over serial
//!
//! This assumes that serial TX is PA0 and RX is PI9. This is true for the
//! art-pi board in which these are connected to the ST-LINK virtual COM
//! port.
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m;
use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*, serial};

use core::fmt::Write;

use nb::block;

#[entry]
fn main() -> ! {
    let _cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let mut ccdr = rcc.sys_ck(100.mhz()).freeze(vos, &dp.SYSCFG);

    // Acquire the GPIOA peripheral. This also enables the clock for
    let gpioa = dp.GPIOA.split(&mut ccdr.ahb4);
    // Acquire the GPIOI peripheral. This also enables the clock for
    let gpioi = dp.GPIOI.split(&mut ccdr.ahb4);

    // initialize serial
    let tx = gpioa.pa0.into_alternate_af8();
    let rx = gpioi.pi9.into_alternate_af8();

    let mut serial = dp
        .UART4
        .usart((tx, rx), serial::config::Config::default().baudrate(115_200_u32.bps()), &mut ccdr)
        .unwrap();

    serial.listen(serial::Event::Rxne);
    let (mut tx, mut rx) = serial.split();

    // core::fmt::Write is implemented for tx.
    writeln!(tx, "Hello, world!").unwrap();

    loop {
        // Echo what is received on the serial link.
        let received = block!(rx.read()).unwrap();
        block!(tx.write(received)).ok();
    }
}
