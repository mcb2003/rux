//! The paging (I.E. virtual memory) system.
//! # Important
//! For this module to be safe, the following
//! invariants must *always* be upheld:
//! * The 511th entry of the active P4 table must always be mapped to the active P4 table itself.

#![allow(dead_code)]
mod entry;
pub use entry::{Entry, EntryFlags};
mod table;
pub use table::PageTable;

use core::marker::PhantomData;

use super::{Addr, SizedRegion};

/// Specifies a page table level
pub trait PageTableLevel: Copy {}

/// Specifies a page table level where the next level down is another page table
pub trait HigherPageTableLevel: PageTableLevel {
    /// Level of the next page table down
    type NextLevel: PageTableLevel;
}

#[derive(Clone, Copy)]
pub enum Level1 {}
#[derive(Clone, Copy)]
pub enum Level2 {}
#[derive(Clone, Copy)]
pub enum Level3 {}
#[derive(Clone, Copy)]
pub enum Level4 {}

impl PageTableLevel for Level1 {}
impl PageTableLevel for Level2 {}
impl PageTableLevel for Level3 {}
impl PageTableLevel for Level4 {}

impl HigherPageTableLevel for Level2 {
    type NextLevel = Level1;
}

impl HigherPageTableLevel for Level3 {
    type NextLevel = Level2;
}

impl HigherPageTableLevel for Level4 {
    type NextLevel = Level3;
}

/// A virtual page of memory
pub struct Page<S: SizedRegion> {
    start: Addr,
    _marker: PhantomData<S>,
}

impl<S: SizedRegion> Page<S> {
    fn containing_addr(addr: Addr) -> Self {
        Self {
            start: addr.align_down(S::SIZE),
            _marker: PhantomData,
        }
    }
}
