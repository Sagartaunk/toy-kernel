#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]
// Enable `x86-interrupt` calling convention
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod memory;
pub mod serial;
pub mod tests;
pub mod vga_buffer;
use crate::tests::test_panic_handler;
use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{BootInfo, entry_point};

#[cfg(test)]
entry_point!(test_kernel_main);

// Entry point for `cargo tests`.
#[cfg(test)]
pub fn kernel_test_main(_boot_info: &'static BootInfo) -> ! {
    inti();
    test_main();
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

/// Make the cpu halt until the next instruction thus
/// allowing it to enter a sleep state and be more
/// energy efficient
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
