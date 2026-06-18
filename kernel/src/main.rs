// Disable both the `std` library and the `main`
// entry point.

#![no_std]
#![no_main]

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
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &bytes) in DISPLAY.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = bytes;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
