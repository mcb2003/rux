use crate::println;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    crate::gdt::init();
    crate::interrupts::init();
    println!("Hello, world!");
    unsafe {
    *(0xdeadbeaf as *mut u64) = 0xdeadbeaf;
    }
        println!("It didn't crash!");
    loop {}
}

