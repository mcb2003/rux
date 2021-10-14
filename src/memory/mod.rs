mod addr;
pub use addr::Addr;
#[cfg(feature = "frame_alloc_simple")]
mod simple_allocator;
//#[cfg(feature = "frame_alloc_simple")]
//pub use simple_allocator::SimpleFrameAllocator;

use core::marker::PhantomData;

/// Specifies the size of a region (page or frame) of memory.
/// Implemented by [`Size4K`], [`Size2M`], and [`Size1G`].
pub trait SizedRegion {
    /// The size (in bytes) of the memory region
    const SIZE: usize;
}

/// A 4 kibibyte memory Size (page or frame).
pub enum Size4K {}
/// A 2 mebibyte memory Size (page or frame).
pub enum Size2M {}
/// A 1 gibibyte memory Size (page or frame).
pub enum Size1G {}

impl SizedRegion for Size4K {
    const SIZE: usize = 4 * 1024;
}

impl SizedRegion for Size2M {
    const SIZE: usize = 2 * 1024 * 1024;
}

impl SizedRegion for Size1G {
    const SIZE: usize = 1 * 1024 * 1024 * 1024;
}

/// A physical page frame that is guaranteed to be allocated.
pub struct AllocatedFrame<S: SizedRegion> {
    number: usize,
    _marker: PhantomData<S>,
}

trait FrameAllocator<S: SizedRegion> {
    fn allocate(&mut self) -> Option<AllocatedFrame<S>>;
    fn deallocate(&mut self, frame: AllocatedFrame<S>);
}
