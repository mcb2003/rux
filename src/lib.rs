#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(asm)]

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
    unsafe {
    *(0xdeadbeaf as *mut u64) = 0xdeadbeaf;
    }
        println!("It didn't crash!");
    loop {}
}
