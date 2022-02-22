#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(slice_index_methods)]

mod gdt;
mod init;
mod interrupts;
mod memory;
mod output;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel {}", info);

    hlt_loop();
}

pub(crate) fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
