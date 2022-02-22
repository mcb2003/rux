use core::marker::PhantomData;

use crate::memory::{
    paging::{Level1, Level2, Level3, Level4, PageTableLevel},
    phys::{AllocatedFrame, SizedAllocatedFrame},
    Addr, Size1G, Size2M, Size4K,
};

/// An entry in a page table
#[derive(Clone, Copy)]
pub struct Entry<L: PageTableLevel> {
    inner: u64,
    _marker: PhantomData<L>,
}

impl<L: PageTableLevel> Default for Entry<L> {
    fn default() -> Self {
        Self {
            inner: 0,
            _marker: PhantomData,
        }
    }
}

impl<L: PageTableLevel> Entry<L> {
    pub fn is_unused(&self) -> bool {
        self.inner == 0
    }

    pub fn set_unused(&mut self) {
        self.inner = 0;
    }

    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.inner)
    }

    pub fn addr(&self) -> Addr {
        Addr::from(self.inner & 0x000fffff_fffff000)
    }
}

impl Entry<Level1> {
    pub fn frame(&self) -> Option<AllocatedFrame<Size4K>> {
        if self.flags().contains(EntryFlags::PRESENT) {
            // Safety: The frame must've been allocated. This is safe if page -> frame mappings are
            // created safely, because you can only map allocated frames
            Some(unsafe { AllocatedFrame::containing_addr(self.addr()) })
        } else {
            None
        }
    }
}

impl Entry<Level2> {
    pub fn frame(&self) -> Option<SizedAllocatedFrame<Size2M>> {
        if self.flags().contains(EntryFlags::PRESENT) {
            // Safety: The frame must've been allocated. This is safe if page -> frame mappings are
            // created safely, because you can only map allocated frames
            Some(unsafe {
                SizedAllocatedFrame::new(self.addr(), self.flags().contains(EntryFlags::HUGE))
            })
        } else {
            None
        }
    }
}

impl Entry<Level3> {
    pub fn frame(&self) -> Option<SizedAllocatedFrame<Size1G>> {
        if self.flags().contains(EntryFlags::PRESENT) {
            // Safety: The frame must've been allocated. This is safe if page -> frame mappings are
            // created safely, because you can only map allocated frames
            Some(unsafe {
                SizedAllocatedFrame::new(self.addr(), self.flags().contains(EntryFlags::HUGE))
            })
        } else {
            None
        }
    }
}

impl Entry<Level4> {
    pub fn frame(&self) -> Option<AllocatedFrame<Size4K>> {
        if self.flags().contains(EntryFlags::PRESENT) {
            // Safety: The frame must've been allocated. This is safe if page -> frame mappings are
            // created safely, because you can only map allocated frames
            Some(unsafe { AllocatedFrame::containing_addr(self.addr()) })
        } else {
            None
        }
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
