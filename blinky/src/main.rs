//! STM32F103 - Blink a LED
//!
//! Code assumes internal LED connected to PC13 as in the blue pill board.
//!
//! Note: non-internal LED connected to PC13 pin needs additional hardware
//! see page 5.1.2 of the reference manual for an explanation.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32f1xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
    // Adquire core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();

    // Adquire device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Own raw flash and rcc devices, then convert them into HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the system clock config and store their frequencies in `clocks`
    let clocks = rcc.cfgr.sysclk(72.MHz()).freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();

    // Setup GPIOC pin 13. The `crh` register should be used for pins 8-15
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

    // Setup the syst timer to trigger updates
    let timer = Timer::syst(cp.SYST, &clocks);
    let mut delay = timer.delay();

    // Wait for the timer update and then change the state of the LED
    loop {
        led.set_high();
        delay.delay_us(1_000_000u32);
        led.set_low();
        delay.delay_us(1_000_000u32);
    }
}