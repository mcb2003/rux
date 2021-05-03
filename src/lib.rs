#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let long: u64 = 0x2f472f4e2f4f2f4c;
    let vga_buf = 0xb8000 as *mut u64;
    unsafe { *vga_buf = long; }
    loop {}
}
