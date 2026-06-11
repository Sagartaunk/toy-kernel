// Disable both the `std` library and the `main`
// entry point.

#![no_std]
#![no_main]

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
    loop {
        // todo!();
    }
}
