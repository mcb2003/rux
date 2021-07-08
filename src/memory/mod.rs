#[cfg(feature = "frame_alloc_simple")]
mod simple_allocator;
#[cfg(feature = "frame_alloc_simple")]
pub use simple_allocator::SimpleFrameAllocator as FrameAllocator;

use x86_64::structures::paging::frame::{PhysFrameRangeInclusive, PhysFrame};

trait PhysFrameRangeExt {
    fn contains(&self, frame: PhysFrame) -> bool;
}

impl PhysFrameRangeExt for PhysFrameRangeInclusive {
    fn contains(&self, frame: PhysFrame) -> bool {
        self.start <= frame && frame <= self.end
    }
}

#[macro_export]
macro_rules! frame_containing {
    ($addr: expr) => {
        PhysFrame::containing_address(x86_64::PhysAddr::new($addr))
    }
}

#[macro_export]
macro_rules! frame_starting_at {
    ($addr: expr) => {
        PhysFrame::from_start_address(x86_64::PhysAddr::new($addr))
    }
}
