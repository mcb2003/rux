use x86_64::{structures::tss::TaskStateSegment, VirtAddr};
use lazy_static::lazy_static;

/// Index of the stack used when a double-fault occurs.
pub const DOUBLE_FAULT_IST_INDEX: u16 = 6;
const STACK_SIZE: usize = 4096 * 5;

/// Storage space for the double-fault stack.
/// Todo: Replace this with allocated memory with a propper guard page.
static mut DOUBLE_FAULT_STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

lazy_static! {
    pub(super) static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {

            let stack_start = VirtAddr::from_ptr(unsafe { &DOUBLE_FAULT_STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}