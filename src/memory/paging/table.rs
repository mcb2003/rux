use core::{
    ops::{Index, IndexMut},
    ptr::NonNull,
    slice::SliceIndex,
};

use super::{Entry, EntryFlags, HigherPageTableLevel, Level4, PageTableLevel};

/// The number of entries in a page table
const ENTRY_COUNT: usize = 512;
/// The entry of the p4 table that maps to the p4 table itself.
const RECURSIVE_ENTRY: usize = 511;

/// Pointer to the P4 table, assuming it is mapped recursively
pub const P4: NonNull<PageTable<Level4>> =
    unsafe { NonNull::new_unchecked(0xfffffffffffff000 as *mut _) };

#[repr(align(4096))]
pub struct PageTable<L: PageTableLevel>([Entry<L>; ENTRY_COUNT]);

impl<L: HigherPageTableLevel> PageTable<L> {
    fn next_table_ptr(&self, index: usize) -> Option<NonNull<PageTable<L::NextLevel>>> {
        let flags = self.0[index].flags();
        // Check that there actually *is* a page table their
        if flags.contains(EntryFlags::PRESENT) && !flags.contains(EntryFlags::HUGE) {
            let our_addr = self as *const _ as usize;
            let new_addr = ((our_addr << 9) | (index << 12)) as _;
            // Safety: I mean ... this is raw pointer magic, what do you expect! But seriously, we
            // know this address will never be NULL.
            Some(unsafe { NonNull::new_unchecked(new_addr) })
        } else {
            None
        }
    }
}

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
