mod simple_allocator;
pub use simple_allocator::SimpleFrameAllocator;

use x86_64::structures::paging::frame::{PhysFrameRangeInclusive, PhysFrame};

trait PhysFrameRangeExt {
    fn contains(&self, frame: PhysFrame) -> bool;
}

impl PhysFrameRangeExt for PhysFrameRangeInclusive {
    fn contains(&self, frame: PhysFrame) -> bool {
        self.start <= frame && frame <= self.end
    }
}
