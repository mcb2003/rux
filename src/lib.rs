#![no_std]

mod output;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel {}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    println!("Hello, world!");
    loop {}
}
