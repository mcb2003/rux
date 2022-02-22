use core::{fmt, ops};

use derive_more::{From, Into};

/// A memory address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From, Into)]
#[repr(transparent)]
pub struct Addr(pub usize);

#[allow(dead_code)]
impl Addr {
    pub const NULL: Self = Self(0);

    /// Returns whether this address is considered to be `NULL`. Note that, even if this function
    /// returns `true`, it may still be possible to use this address, depending on how the page
    /// tables or physical frames are set up.
    pub fn is_null(self) -> bool {
        self == Self::NULL
    }

    /// Cast this address to a const pointer of the specified type.
    /// # Safety
    /// Use of this pointer requires that it is a valid pointer to something of the correct type.
    pub unsafe fn as_ptr<T>(self) -> *const T {
        self.0 as _
    }

    /// Cast this address to a mutable pointer of the specified type.
    /// # Safety
    /// Use of this pointer requires that it is a valid pointer to something of the correct type.
    pub unsafe fn as_mut_ptr<T>(self) -> *mut T {
        self.0 as _
    }

    /// Aligns the address to `alignment` (E.G. 4096) downwards.
    /// #[inline]
    pub fn align_down(self, alignment: usize) -> Self {
        Self(self.0 & !(alignment - 1))
    }

    /// Aligns the address to `alignment` (E.G. 4096) upwards.
    #[inline]
    pub fn align_up(self, alignment: usize) -> Self {
        Self(self.0 + alignment).align_down(alignment)
    }
}

#[cfg(target_pointer_width = "64")]
impl From<u64> for Addr {
    fn from(address: u64) -> Self {
        Self(address as usize)
    }
}

#[cfg(target_pointer_width = "32")]
impl From<u32> for Addr {
    fn from(address: u32) -> Self {
        Self(address as usize)
    }
}

#[cfg(target_pointer_width = "64")]
impl From<Addr> for u64 {
    fn from(addr: Addr) -> Self {
        addr.0 as _
    }
}

#[cfg(target_pointer_width = "32")]
impl From<Addr> for u32 {
    fn from(addr: Addr) -> Self {
        addr.0 as _
    }
}

impl fmt::Debug for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#16x}", self.0)
    }
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl ops::Add<usize> for Addr {
    type Output = Self;

    fn add(self, other: usize) -> Self::Output {
        Self(self.0 + other)
    }
}

impl ops::AddAssign<usize> for Addr {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}
