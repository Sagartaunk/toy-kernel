//! This is the entry point of the kernel
//! and glues togeather all the functionality.

//! Disable both the `std` library and the `main`
//! entry point.
//!
//! Declare custom test suit that does not require
//! `std` crate.
#![no_std]
#![no_main]
// Declare custom test framework and re-export it as
// `test_main` so we can call it in other places. This is
// because by default the compiler exports the test framework
// as `main` and due to the `no_main` attribute in `main.rs`
// we ignore the `main` function and instead use `_start`
// as the entry point of our binary.
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

// Import display module.
mod tests;
mod vga_buffer;
use core::panic::PanicInfo;

#[cfg(test)]
use crate::tests::Testable;
mod serial;

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

/// This panic handeller only woks in test mode.
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    tests::exit_qemu(tests::QemuExitCode::Failed);
    loop {}
}

/// Entry point of the binary.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_println!("Hello World{}", "!");

    kernel::init();

    #[cfg(test)]
    test_main();

    println!("Works till here most likely");
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }

    tests::exit_qemu(tests::QemuExitCode::Success);
}
