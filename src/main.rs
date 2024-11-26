#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm::nop;
use cortex_m_rt::entry;

use core::ptr::read_volatile;
use core::ptr::write_volatile;
const GPIO_OE: usize = 0x20;
const GPIO_OUT_SET: usize = 0x14;
const GPIO_OUT_CLEAR: usize = 0x18;
const SIO_BASE_ADDR: usize = 0xD0000000;

const IO_BANK0: usize = 0x40014000;
const GPIO15_CTRL: usize = 0x7C;
const RESET_DONE_REG: usize = 0x4000C008;
const GPIO_15: usize = 15;

fn read_register(address: usize) -> u32 {
    unsafe { read_volatile(address as *const u32) }
}

fn write_register(address: usize, value: u32) {
    unsafe { write_volatile(address as *mut u32, value) }
}

#[entry]
fn main() -> ! {
    write_register(IO_BANK0, 1 << 5); // Set SIO function of the GPIO pins function MUX by setting value to 5

    while read_register(RESET_DONE_REG) & (1 << 5) == 0 {
        nop();
    }

    // Set GPIO15 function to 5 (SIO control)
    write_register(IO_BANK0 + GPIO15_CTRL, 1 << 5);

    // Set GPIO15 as an output pin
    write_register(SIO_BASE_ADDR + GPIO_OE, 1 << GPIO_15);
    loop {
        // Turn GPIO15 (LED) ON
        write_register(SIO_BASE_ADDR + GPIO_OUT_SET, 1 << GPIO_15);

        // Delay for the blink
        for _ in 0..1_000_000 {
            nop();
        }

        // Turn GPIO15 (LED) OFF
        write_register(SIO_BASE_ADDR + GPIO_OUT_CLEAR, 1 << GPIO_15);

        // Delay for the blink
        for _ in 0..1_000_000 {
            nop();
        }
    }
}
