//! This module contains dependencies of the test
//! harness of the kernel as well as the test cases

use crate::hlt_loop;
use crate::print;
use crate::println;
use crate::serial_print;
use crate::serial_println;
use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
/// Safely exit the `QEMU` system.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {
        hlt_loop();
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

pub trait Testable {
    fn run(&self) -> ();
}

/// Automatically print status information for test cases
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}", core::any::type_name::<T>());
        self();
        serial_println!("[OK]");
    }
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
