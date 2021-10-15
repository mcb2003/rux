#[cfg(feature = "frame_alloc_simple")]
mod simple_allocator;
#[cfg(feature = "frame_alloc_simple")]
pub use simple_allocator::SimpleFrameAllocator;

use core::{fmt, marker::PhantomData};

use crate::memory::{Addr, SizedRegion};

/// A physical page frame that is guaranteed to be allocated.
pub struct AllocatedFrame<S: SizedRegion> {
    start: Addr,
    _marker: PhantomData<S>,
}

impl<S: SizedRegion> AllocatedFrame<S> {
    unsafe fn containing_addr(addr: Addr) -> Self {
        Self {
            start: addr.align_down(S::SIZE),
            _marker: PhantomData,
        }
    }
}

impl<S: SizedRegion> fmt::Debug for AllocatedFrame<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Frame")
            .field(&self.start)
            .field(&S::DISPLAY)
            .finish()
    }
}

pub trait FrameAllocator<S: SizedRegion>: Iterator<Item = AllocatedFrame<S>> {
    fn deallocate(&mut self, frame: AllocatedFrame<S>);
}

trait MemoryAreaExt {
    fn contains(&self, addr: Addr) -> bool;
}

impl MemoryAreaExt for multiboot2::MemoryArea {
    fn contains(&self, addr: Addr) -> bool {
        Addr::from(self.end_address()) > addr && addr >= Addr::from(self.start_address())
    }
}
