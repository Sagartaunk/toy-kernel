//! This module contains intrrupt handelers.

#![feature(abi_x86_interrupt)]

use crate::{gdt, println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

/// Define an Interrupt descriptor table.
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
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

pub const PIC1_OFFSET: u8 = 32;
pub const PIC2_OFFSET: u8 = 1;
// SAFETY: Thr pic's are placed at valid addresses starting from 32.
// Thus, leaving all the bits used for exception are seperate.
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC1_OFFSET, PIC2_OFFSET) });
