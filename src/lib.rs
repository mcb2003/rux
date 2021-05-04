#![no_std]
#![feature(abi_x86_interrupt)]

mod output;
mod interrupts;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    interrupts::init();
    println!("Hello, world!");
    loop {}
}
