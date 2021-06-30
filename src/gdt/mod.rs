pub mod tss;

use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

/// A struct wrapping a GDT and a CS and TSS segment selector
struct GdtWrapper {
    gdt: GlobalDescriptorTable,
    code_sel: SegmentSelector,
    tss_sel: SegmentSelector,
}

lazy_static! {
    static ref GDT: GdtWrapper = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_sel = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_sel = gdt.add_entry(Descriptor::tss_segment(&tss::TSS));
        GdtWrapper {
            gdt,
            code_sel,
            tss_sel,
        }
    };
}

pub fn init() {
    use x86_64::instructions::{segmentation::set_cs, tables::load_tss};

    // Load the GDT itself
    GDT.gdt.load();
    unsafe {
        set_cs(GDT.code_sel); // Reload CS
        load_tss(GDT.tss_sel);
    }
}
