#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

mod gdt;
mod interrupts;
mod output;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    gdt::init();
    interrupts::init();
    println!("Hello, world!");
    unsafe {
    *(0xdeadbeaf as *mut u64) = 0xdeadbeaf;
    }
        println!("It didn't crash!");
    loop {}
}
