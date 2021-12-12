use core::marker::PhantomData;

use crate::memory::{Addr, SizedRegion};

/// The number of entries in a page table
const ENTRY_COUNT: usize = 512;

/// An entry in a page table
#[derive(Default)]
pub struct Entry(u64);

impl Entry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    pub fn addr(&self) -> Addr {
        Addr::from(self.0 & 0x000fffff_fffff000)
    }
}

bitflags::bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT = 1 << 0;
        const WRITABLE = 1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH = 1 << 3;
        const NO_CACHE = 1 << 4;
        const ACCESSED = 1 << 5;
        const DIRTY = 1 << 6;
        const HUGE = 1 << 7;
        const GLOBAL = 1 << 8;
        const NO_EXEC = 1 << 63;
    }
}

#[allow(dead_code)]
impl EntryFlags {
    pub fn readable(self) -> bool {
        self.contains(Self::PRESENT)
    }

    pub fn writable(self) -> bool {
        self.contains(Self::PRESENT | Self::WRITABLE)
    }

    pub fn executable(self) -> bool {
        self.contains(Self::PRESENT) && !self.contains(Self::NO_EXEC)
    }
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
