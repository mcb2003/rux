use super::{Entry, PageTableLevel};

/// The number of entries in a page table
const ENTRY_COUNT: usize = 512;
/// The entry of the p4 table that maps to the p4 table itself.
const RECURSIVE_ENTRY: usize = 511;

#[repr(align(4096))]
pub struct PageTable<L: PageTableLevel>([Entry<L>; ENTRY_COUNT]);

impl<L: PageTableLevel> Default for PageTable<L> {
    fn default() -> Self {
        Self([Entry::default(); ENTRY_COUNT])
    }
}
