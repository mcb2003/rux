use core::ops::Range;

use multiboot2::{MemoryArea, MemoryMapTag};

use super::{AllocatedFrame, FrameAllocator, MemoryAreaExt};
use crate::memory::{Addr, Size4K, SizedRegion};

pub struct SimpleFrameAllocator<'a> {
    next: Addr,
    memory_map: &'a MemoryMapTag,
    current_area: Option<&'a MemoryArea>,
    kernel: Range<Addr>,
    multiboot_info: Range<Addr>,
}

impl<'a> SimpleFrameAllocator<'a> {
    pub fn new(mb: &'a multiboot2::BootInformation) -> Self {
        let sections = || {
            mb.elf_sections_tag()
                .expect("Multiboot2 ELF sections tag required")
                .sections()
        };

        let kernel_start = Addr::from(sections().map(|a| a.start_address()).min().unwrap());
        let kernel_end = Addr::from(sections().map(|a| a.end_address()).max().unwrap());

        let multiboot_start = Addr::from(mb.start_address());
        let multiboot_end = Addr::from(mb.end_address());

        let mut allocator = Self {
            next: Addr::from(0_usize),
            memory_map: mb
                .memory_map_tag()
                .expect("Multiboot2 memory map tag required"),
            current_area: None,
            kernel: (kernel_start..kernel_end).into(),
            multiboot_info: multiboot_start..multiboot_end,
        };
        allocator.next_area();
        allocator
    }

    fn next_area(&mut self) {
        self.current_area = self
            .memory_map
            .memory_areas()
            .filter(|area| {
                let address = Addr::from(area.end_address() - 1);
                address >= self.next
            })
            .min_by_key(|area| area.start_address());
    }
}

impl Iterator for SimpleFrameAllocator<'_> {
    type Item = AllocatedFrame<Size4K>;

    fn next(&mut self) -> Option<Self::Item> {
        let area = self.current_area?;
        let next = if self.kernel.contains(&self.next) {
            self.kernel.end.align_up(Size4K::SIZE)
        } else if self.multiboot_info.contains(&self.next) {
            self.multiboot_info.end.align_up(Size4K::SIZE)
        } else {
            self.next
        };
        self.next = next + Size4K::SIZE;
        if !area.contains(next) {
            self.next_area();
            self.next()
        } else {
            Some(unsafe { AllocatedFrame::containing_addr(next) })
        }
    }
}

impl FrameAllocator<Size4K> for SimpleFrameAllocator<'_> {
    fn deallocate(&mut self, _frame: AllocatedFrame<Size4K>) {
        unimplemented!()
    }
}
