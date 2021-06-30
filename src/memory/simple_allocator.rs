use core::ops::RangeInclusive;

use multiboot2::{MemoryArea, MemoryAreaIter, MemoryAreaType};

use super::{Frame, FrameAllocator};

pub struct SimpleFrameAllocator<'a> {
    next: Frame,
    areas: MemoryAreaIter<'a>,
    current_area: Option<&'a MemoryArea>,
    kernel: RangeInclusive<Frame>,
    multiboot: RangeInclusive<Frame>,
    
}

impl<'a> SimpleFrameAllocator<'a> {
    pub fn new(mb: &'a multiboot2::BootInformation) -> Self {
        let kernel_start = Frame::containing_address(mb.elf_sections_tag().expect("Multiboot2 ELF sections tag required").sections().map(|s| s.start_address() as usize).min().unwrap()); // Should always sections
        let kernel_end = Frame::containing_address(mb.elf_sections_tag().expect("Multiboot2 ELF sections tag required").sections().map(|s| s.end_address() as usize - 1).max().unwrap());
        let mut allocator = Self {
            next: Frame(0),
            areas: mb.memory_map_tag().expect("Multiboot2 memory areas tag required").all_memory_areas(),
            current_area: None,
            kernel: kernel_start..=kernel_end,
            multiboot: Frame::containing_address(mb.start_address())..=Frame::containing_address(mb.end_address()),
        };
        allocator.next_area();
        allocator
    }
    fn next_area(&mut self) {
            self.current_area = self.areas.clone().filter(|area| {
        let address = area.end_address() - 1;
        area.typ() == MemoryAreaType::Available && Frame::containing_address(address as usize) >= self.next
    }).min_by_key(|area| area.start_address());

    if let Some(area) = self.current_area {
        let start_frame = Frame::containing_address(area.start_address() as usize);
        if self.next < start_frame {
            self.next = start_frame;
        }
    }
    }
}

impl<'a> Iterator for SimpleFrameAllocator<'a> {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if let Some(area) = self.current_area {
            let frame = Frame(self.next.0);
            let last = Frame::containing_address(area.end_address() as usize - 1);
            if frame > last {
                // Allocate from the next area of memory
                self.next_area();
            } else if self.kernel.contains(&frame) {
                self.next = Frame(self.kernel.end().0 + 1);
            } else if self.multiboot.contains(&frame) {
                self.next = Frame(self.multiboot.end().0 + 1);
            } else {
                self.next.0 += 1;
            return Some(frame);
            }
            self.next() // Try again with the new next frame
        } else {
            None // No more free frames
        }
    }
}

impl<'a> FrameAllocator for SimpleFrameAllocator<'a> {
    fn dealloc(&mut self, _frame: Frame) {
        unimplemented!();
    }
}
