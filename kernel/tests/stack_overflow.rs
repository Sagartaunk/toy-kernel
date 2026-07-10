//! This module contains a stack overflow test suit.
#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use kernel::serial_print;
use kernel::tests::test_panic_handler;
use kernel::{
    serial_println,
    tests::{QemuExitCode, exit_qemu},
};
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial_print!("Stack_Overflow_Test::Running");
    kernel::gdt::init();
    init_test_idt();
    // Trigger a double fault.
    stack_overflow();
    // The test never intends to reach this line.
    panic!("Exectuion continued after stack overflow");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    // Triggers infinite recursion by calling itself.
    // Hence, triggering a double fault in the kernel.
    stack_overflow();
    // Prevents optimisation for tail recursion.
    volatile::Volatile::new(0).read();
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_fault_handler)
                .set_stack_index(kernel::gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub fn init_test_idt() {
    TEST_IDT.load();
}

extern "x86-interrupt" fn test_double_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
