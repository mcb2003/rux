use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt
    };
}

pub(super) fn init() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(info: InterruptStackFrame) {
    println!("Breakpoint: {:#?}", info);
}

extern "x86-interrupt" fn divide_error_handler(info: InterruptStackFrame) {
    panic!("div by 0: {:#?}", info);
}
