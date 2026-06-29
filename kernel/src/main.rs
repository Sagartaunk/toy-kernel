//! This is the entry point of the kernel
//! and glues togeather all the functionality.

//! Disable both the `std` library and the `main`
//! entry point.
#![no_std]
#![no_main]

// Import display module.
mod vga_buffer;

use core::panic::PanicInfo;

/// When something panic's this function is called.
/// It is disabled to compile on test cases because
/// they implicitly include the `std` crate.
///
/// The function depends on the `println` macro defined
/// in `vga_buffer.rs`.
#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// Entry point of the binary.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}
