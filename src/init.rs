use crate::{memory::SimpleFrameAllocator, println};

const KERNEL_NAME: &str = env!("CARGO_PKG_NAME");

const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");

#[no_mangle]

pub extern "C" fn kernel_main(multiboot_info: usize) -> ! {
    crate::gdt::init();

    crate::interrupts::init();

    let multiboot_info = unsafe { multiboot2::load(multiboot_info) };

    let bootloader = multiboot_info
        .boot_loader_name_tag()
        .map(|t| t.name())
        .unwrap_or("unknown bootloader");

    println!(
        "Starting {} kernel v.{}, booted by {:?}",
        KERNEL_NAME, KERNEL_VERSION, bootloader
    );

    let cmdline = multiboot_info
        .command_line_tag()
        .map(|t| t.command_line())
        .unwrap_or("");

    println!("Kernel cmdline: {:?}", cmdline);

    print_memory_areas(&multiboot_info);
    print_elf_sections(&multiboot_info);

    let fa = SimpleFrameAllocator::new(&multiboot_info);
    for frame in fa.skip(159).take(10) {
        println!("{:?}", frame);
    }

    crate::hlt_loop();
}

fn print_memory_areas(mb: &multiboot2::BootInformation) {
    let mm_tag = mb
        .memory_map_tag()
        .expect("Multiboot2 structure must have a memory map tag");

    println!("memory areas:");

    println!("{:17} {:10} {:10} Length", "Type", "Start", "End");

    for area in mm_tag.memory_areas() {
        println!(
            "{:17?} {:#010x} {:#010x} {}",
            area.typ(),
            area.start_address(),
            area.end_address(),
            area.size()
        );
    }
}

fn print_elf_sections(mb: &multiboot2::BootInformation) {
    let elf_tag = mb
        .elf_sections_tag()
        .expect("Multiboot2 structure must have an ELF sections tag");

    println!("ELF Sections:");

    println!(
        "{:24} {:10} {:10} {:10} Name",
        "Type", "Start", "End", "Flags",
    );

    for section in elf_tag.sections() {
        println!(
            "{:24?} {:#010x} {:#010x} {:10?} {}",
            section.section_type(),
            section.start_address(),
            section.end_address(),
            section.flags(),
            section.name()
        );
    }
}
