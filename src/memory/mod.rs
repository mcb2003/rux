mod addr;
pub use addr::Addr;
pub mod paging;
pub mod phys;

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
