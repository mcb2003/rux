use crate::println;

const KERNEL_NAME: &'static str = env!("CARGO_PKG_NAME");
const KERNEL_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn kernel_main(multiboot_info: usize) -> ! {
    crate::gdt::init();
    crate::interrupts::init();
    let multiboot_info = unsafe { multiboot2::load(multiboot_info) };
    let bootloader = multiboot_info.boot_loader_name_tag().map(|t| t.name()).unwrap_or("unknown bootloader");
    println!("Starting {} kernel v.{}, booted by {}", KERNEL_NAME, KERNEL_VERSION, bootloader);
    crate::hlt_loop();
}
