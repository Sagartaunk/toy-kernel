//! This module contains intrrupt handelers.

#![feature(abi_x86_interrupt)]

use crate::println;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use lazy_static::lazy_static;

/// Define an Interrupt descriptor table.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.double_fault.set_handler_fn(double_fault_handler);
        idt
    };
}

/// Intialise the Interrupt Descriptor table.
pub fn init_idt() {
    IDT.load();
}

/// This handeler just pretty-prints the interrupt stack frame.
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

/// Pretty-prints the exception stack frame for a double fault.
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("Exception: DOUBLE FAULT\n{:#?}", stack_frame);
}
