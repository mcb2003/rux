#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

mod gdt;
mod init;
mod interrupts;
mod output;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel {}", info);
    loop {}
}
