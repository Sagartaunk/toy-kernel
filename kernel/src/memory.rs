//! This module contains the memory management
//! implementations.

use x86_64::{VirtAddr, structures::paging::PageTable};

/// Return a mutable reference to the active `level 4` table.
///
/// SAFETY: The caller must gurantee that the complete physical
/// memory is mapped to the virtual memory at the passed
/// `physical_memory_offset`.
///
/// SAFETY: This function must only be called once to avoid
/// `&mut` which will trigger an undefined behaviour.

pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}
