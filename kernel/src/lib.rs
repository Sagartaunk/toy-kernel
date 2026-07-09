#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
// Enable `x86-interrupt` calling convention
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod serial;
pub mod tests;
pub mod vga_buffer;
use crate::tests::test_panic_handler;
use core::panic::PanicInfo;

// Entry point for `cargo tests`.
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}
