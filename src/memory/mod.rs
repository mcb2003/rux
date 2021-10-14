mod addr;
pub use addr::Addr;
#[cfg(feature = "frame_alloc_simple")]
mod simple_allocator;
#[cfg(feature = "frame_alloc_simple")]
pub use simple_allocator::SimpleFrameAllocator;

use core::{fmt, marker::PhantomData};

/// Specifies the size of a region (page or frame) of memory.
/// Implemented by [`Size4K`], [`Size2M`], and [`Size1G`].
pub trait SizedRegion {
    /// The size (in bytes) of the memory region
    const SIZE: usize;
    /// A human-readable representation of the size
    const DISPLAY: &'static str;
}

/// A 4 kibibyte memory Size (page or frame).
pub enum Size4K {}
/// A 2 mebibyte memory Size (page or frame).
pub enum Size2M {}
/// A 1 gibibyte memory Size (page or frame).
pub enum Size1G {}

impl SizedRegion for Size4K {
    const SIZE: usize = 4 * 1024;
    const DISPLAY: &'static str = "4K";
}

impl SizedRegion for Size2M {
    const SIZE: usize = 2 * 1024 * 1024;
    const DISPLAY: &'static str = "2M";
}

impl SizedRegion for Size1G {
    const SIZE: usize = 1 * 1024 * 1024 * 1024;
    const DISPLAY: &'static str = "1G";
}

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

pub trait FrameAllocator<S: SizedRegion> {
    fn allocate(&mut self) -> Option<AllocatedFrame<S>>;
    fn deallocate(&mut self, frame: AllocatedFrame<S>);
}

trait MemoryAreaExt {
    fn contains(&self, addr: addr::Addr) -> bool;
}

impl MemoryAreaExt for multiboot2::MemoryArea {
    fn contains(&self, addr: Addr) -> bool {
        Addr::from(self.end_address()) > addr && addr >= Addr::from(self.start_address())
    }
}
