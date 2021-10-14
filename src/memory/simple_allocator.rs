use core::ops::Range;

use multiboot2::{MemoryArea, MemoryAreaIter, MemoryAreaType};

use super::*;

#[derive(Debug)]
pub struct SimpleFrameAllocator<'a> {
    next: Addr,
    areas: MemoryAreaIter<'a>,
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
            areas: mb
                .memory_map_tag()
                .expect("No multiboot memory map tag provided")
                .all_memory_areas(),
            current_area: None,
            kernel: (kernel_start..kernel_end).into(),
            multiboot_info: multiboot_start..multiboot_end,
        };
        allocator.next_area();
        allocator
    }

    fn next_area(&mut self) {
        self.current_area = self
            .areas
            .clone()
            .filter(|area| {
                let address = Addr::from(area.end_address() - 1);
                area.typ() == MemoryAreaType::Available && address >= self.next
            })
            .min_by_key(|area| area.start_address());
    }
}

impl FrameAllocator<Size4K> for SimpleFrameAllocator<'_> {
    fn allocate(&mut self) -> Option<AllocatedFrame<Size4K>> {
        let area = self.current_area?;
        let next = if self.kernel.contains(&self.next) {
            self.kernel.end
        } else if self.multiboot_info.contains(&self.next) {
            self.multiboot_info.end
        } else {
            self.next
        };
        if area.contains(next) {
            self.next = next + Size4K::SIZE;
            // Safety: We have already established that this frame is unused
            Some(unsafe { AllocatedFrame::containing_addr(next) })
        } else {
            self.next_area();
            // I sure hope this can be tail recursed! It definitely should be possible
            self.allocate()
        }
    }

    fn deallocate(&mut self, _frame: AllocatedFrame<Size4K>) {
        unimplemented!()
    }
}
