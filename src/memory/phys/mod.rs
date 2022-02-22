#[cfg(feature = "frame_alloc_simple")]
mod simple_allocator;
#[cfg(feature = "frame_alloc_simple")]
pub use simple_allocator::SimpleFrameAllocator;

use core::{fmt, marker::PhantomData};

use crate::memory::{Addr, Size4K, SizedRegion};

/// A physical page frame that is guaranteed to be allocated.
pub struct AllocatedFrame<S: SizedRegion> {
    start: Addr,
    _marker: PhantomData<S>,
}

impl<S: SizedRegion> AllocatedFrame<S> {
    pub unsafe fn containing_addr(addr: Addr) -> Self {
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

/// An [`AllocatedFrame`] that is either a normal frame, or a "huge" frame of a certain size
pub enum SizedAllocatedFrame<S: SizedRegion> {
    Normal(AllocatedFrame<Size4K>),
    Huge(AllocatedFrame<S>),
}

impl<S: SizedRegion> SizedAllocatedFrame<S> {
    pub unsafe fn new(addr: Addr, huge: bool) -> Self {
        if huge {
            Self::Huge(AllocatedFrame::containing_addr(addr))
        } else {
            Self::Normal(AllocatedFrame::containing_addr(addr))
        }
    }
}

trait MemoryAreaExt {
    fn contains(&self, addr: Addr) -> bool;
}

impl MemoryAreaExt for multiboot2::MemoryArea {
    fn contains(&self, addr: Addr) -> bool {
        Addr::from(self.end_address()) > addr && addr >= Addr::from(self.start_address())
    }
}
