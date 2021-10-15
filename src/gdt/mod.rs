pub mod tss;

use spin::Lazy;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

/// A struct wrapping a GDT and a CS and TSS segment selector

struct GdtWrapper {
    gdt: GlobalDescriptorTable,
    code_sel: SegmentSelector,
    tss_sel: SegmentSelector,
}

static GDT: Lazy<GdtWrapper> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();

    let code_sel = gdt.add_entry(Descriptor::kernel_code_segment());

    let tss_sel = gdt.add_entry(Descriptor::tss_segment(&tss::TSS));

    GdtWrapper {
        gdt,
        code_sel,
        tss_sel,
    }
});

pub fn init() {
    use x86_64::instructions::{
        segmentation::{Segment, CS},
        tables::load_tss,
    };

    // Load the GDT itself
    GDT.gdt.load();

    unsafe {
        CS::set_reg(GDT.code_sel); // Reload CS
        load_tss(GDT.tss_sel);
    }
}
