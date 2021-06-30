use crate::println;

const KERNEL_NAME: &'static str = env!("CARGO_PKG_NAME");
const KERNEL_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[no_mangle]
pub extern "C" fn kernel_main(multiboot_info: usize) -> ! {
    crate::gdt::init();
    crate::interrupts::init();
    let multiboot_info = unsafe { multiboot2::load(multiboot_info) };
    let bootloader = multiboot_info.boot_loader_name_tag().map(|t| t.name()).unwrap_or("unknown bootloader");
    println!("Starting {} kernel v.{}, booted by {:?}", KERNEL_NAME, KERNEL_VERSION, bootloader);
    let cmdline = multiboot_info.command_line_tag().map(|t| t.command_line()).unwrap_or("");
    println!("Kernel cmdline: {:?}", cmdline);
    crate::hlt_loop();
}
