//! This is the entry point of the kernel
//! and glues togeather all the functionality.

//! Disable both the `std` library and the `main`
//! entry point.
#![no_std]
#![no_main]

// Import display module.
mod vga_buffer;

/// Convert test to bytes before displaying.
static DISPLAY: &[u8] = b"SAGAR TAUNK";

use core::panic::PanicInfo;

/// When something panic's this function is called.
/// It is disabled to compile on test cases because
/// they implicitly include the `std` crate.
#[cfg(not(test))]
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {
        //todo!();
    }
}

/// Entry point of the binary.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    // Define memory address for the display buffer.
    // The address `0xb8000` is assigned to the display
    // buffer and the `vga` card listens to changes in
    // this address range.
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("This will be printed through the new function")
        .unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    loop {}
}
