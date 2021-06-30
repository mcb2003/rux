use core::ops::Range;

use super::FRAME_SIZE;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(pub(super) usize);

impl Frame {
    pub const fn containing_address(addr: usize) -> Frame {
        Self(addr / FRAME_SIZE)
    }

    #[inline]
    pub fn start_address(&self) -> usize {
        self.0 * FRAME_SIZE
    }

    #[inline]
    pub fn end_address(&self) -> usize {
        self.start_address() + FRAME_SIZE
    }

    pub fn range(&self) -> Range<usize> {
        self.start_address()..self.end_address()
    }
}
