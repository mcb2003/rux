use multiboot2::{MemoryArea, MemoryAreaIter, MemoryAreaType};
use x86_64::structures::paging::{
    frame::PhysFrameRangeInclusive, FrameAllocator, PhysFrame, Size4KiB,
};

use super::PhysFrameRangeExt;
use crate::{frame_containing, frame_starting_at};

pub struct SimpleFrameAllocator<'a> {
    next: PhysFrame,
    areas: MemoryAreaIter<'a>,
    current_area: Option<&'a MemoryArea>,
    kernel: PhysFrameRangeInclusive,
    multiboot: PhysFrameRangeInclusive,
}

impl<'a> SimpleFrameAllocator<'a> {
    pub fn new(mb: &'a multiboot2::BootInformation) -> Self {
        // There should always be at least one ELF section
        let kernel_start = frame_containing!(mb
            .elf_sections_tag()
            .expect("Multiboot2 ELF sections tag required")
            .sections()
            .map(|s| s.start_address())
            .min()
            .unwrap());
        let kernel_end = frame_containing!(mb
            .elf_sections_tag()
            .expect("Multiboot2 ELF sections tag required")
            .sections()
            .map(|s| s.end_address())
            .max()
            .unwrap());
        let mut allocator = Self {
            next: frame_starting_at!(0).unwrap(),
            areas: mb
                .memory_map_tag()
                .expect("Multiboot2 memory areas tag required")
                .all_memory_areas(),
            current_area: None,
            kernel: PhysFrame::range_inclusive(kernel_start, kernel_end),
            multiboot: PhysFrame::range_inclusive(
                frame_containing!(mb.start_address() as u64),
                frame_containing!(mb.end_address() as u64),
            ),
        };
        allocator.next_area();
        allocator
    }

    fn next_area(&mut self) {
        self.current_area = self
            .areas
            .clone()
            .filter(|area| {
                let address = area.end_address() - 1;
                area.typ() == MemoryAreaType::Available && frame_containing!(address) >= self.next
            })
            .min_by_key(|area| area.start_address());

        if let Some(area) = self.current_area {
            let start_frame = frame_containing!(area.start_address());
            if self.next < start_frame {
                self.next = start_frame;
            }
        }
    }
}

impl<'a> Iterator for SimpleFrameAllocator<'a> {
    type Item = PhysFrame;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(area) = self.current_area {
            let frame = self.next;
            let last = frame_containing!(area.end_address());
            if frame > last {
                // Allocate from the next area of memory
                self.next_area();
            } else if self.kernel.contains(frame) {
                self.next = self.kernel.end + 1;
            } else if self.multiboot.contains(frame) {
                self.next = self.multiboot.end + 1;
            } else {
                self.next += 1;
                return Some(frame);
            }
            self.next() // Try again with the new next frame
        } else {
            None // No more free frames
        }
    }
}

unsafe impl<'a> FrameAllocator<Size4KiB> for SimpleFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.next()
    }
}
