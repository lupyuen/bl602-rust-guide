#![no_std]
#![no_main]

use bl602_hal::gpio::{Output, Pin1, PullDown};
use bl602_hal::{pac, prelude::*};

static mut PANIC_LED: Option<Pin1<Output<PullDown>>> = None;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        if let Some(led_pin) = &mut PANIC_LED {
            // Don't try and panic if it fails, as we are `panic`!
            let _ = led_pin.try_set_high();
        }
    }
    loop {}
}

#[riscv_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let parts = dp.GLB.split();

    unsafe {
        PANIC_LED = Some(parts.pin1.into_pull_down_output());
    }

    let mut gpio5 = parts.pin5.into_pull_down_output();
    gpio5.try_set_high().unwrap();
    loop {
        use riscv::register::mcycle;
        let t0 = mcycle::read64();
        if t0 % 7 == 0 {
            panic!("oh no")
        }
        while mcycle::read64().wrapping_sub(t0) <= 50_000_000 {}
        gpio5.try_toggle().unwrap();
    }
}
