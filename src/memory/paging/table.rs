use core::{
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

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

impl<Idx, L> Index<Idx> for PageTable<L>
where
    Idx: SliceIndex<[Entry<L>; ENTRY_COUNT]>,
    L: PageTableLevel,
{
    type Output = Idx::Output;

    fn index(&self, idx: Idx) -> &Idx::Output {
        idx.index(&self.0)
    }
}

impl<Idx, L> IndexMut<Idx> for PageTable<L>
where
    Idx: SliceIndex<[Entry<L>; ENTRY_COUNT]>,
    L: PageTableLevel,
{
    fn index_mut(&mut self, idx: Idx) -> &mut Idx::Output {
        idx.index_mut(&mut self.0)
    }
}
