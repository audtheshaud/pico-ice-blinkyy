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
const GPIO_XOR: usize = 0x2C;
const SIO_BASE_ADDR: usize = 0xD0000000;

const IO_BANK0: usize = 0x40014000;
const GPIO23_CTRL: usize = 0xBC;
const RESET_DONE_REG: usize = 0x4000C008;
const GPIO_23: usize = 23;

fn read_register(address: usize) -> u32 {
    unsafe { read_volatile(address as *const u32) }
}

fn write_register(address: usize, value: u32) {
    unsafe { write_volatile(address as *mut u32, value) }
}

#[entry]
fn main() -> ! {
    // Reset the IO_BANK0
    write_register(0x4000f000, 1 << 5);

    // Wait for reset to complete
    while read_register(RESET_DONE_REG) & (1 << 5) == 0 { nop(); }

    // Set GPIO function to 5 (SIO control)
    write_register(IO_BANK0 + GPIO23_CTRL, 1 << 5);

    // Set GPIO as an output pin
    write_register(SIO_BASE_ADDR + GPIO_OE, 1 << GPIO_23);

    loop {
        // Toggle GPIO (LED) ON
        write_register(SIO_BASE_ADDR + GPIO_XOR, 1 << GPIO_23);

        // Delay for the blink
        for _ in 0..100_000_000 {
            nop();
        }

        /* // Turn GPIO15 (LED) OFF
        write_register(SIO_BASE_ADDR + GPIO_OUT_CLEAR, 1 << GPIO_13);

        // Delay for the blink
        for _ in 0..100_000 {
            nop();
        } */
    }
}
