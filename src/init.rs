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

    let mm_tag = multiboot_info.memory_map_tag()
    .expect("Multiboot2 structure must have a memory map tag");
println!("memory areas:");
println!("{:17} {:10} {:10} {}", "Type", "Start", "End", "Length");
for area in mm_tag.memory_areas() {
    println!("{:17?} {:#010x} {:#010x} {}", area.typ(), area.start_address(), area.end_address(), area.size());
}

let elf_tag = multiboot_info.elf_sections_tag()
    .expect("Multiboot2 structure must have an ELF sections tag");
    println!("ELF Sections:");
    println!("{:24} {:10} {:10} {}", "Type", "Start", "End", "Name");
    for section in elf_tag.sections() {
        println!("{:24?} {:#010x} {:#010x} {}", section.section_type(), section.start_address(), section.end_address(), section.name());
    }
    crate::hlt_loop();
}
