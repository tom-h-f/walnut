use super::{align_down, align_up};
use crate::memory::paging::PAGE_SIZE;

pub trait Addr {
    fn as_u64(self) -> u64;
}

/// Represents a physical address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct PhysAddr(pub u64);

impl PhysAddr {
    /// Creates a instance of `PhysAddr` containing `addr`
    pub fn containing_addr<U: Into<u64>>(addr: u64) -> Self {
        // Align the address down to the closest page
        // start address
        Self(align_down(addr, PAGE_SIZE))
    }

    /// Creates a new physical address, throwing bits 52..64 away.
    #[inline]
    pub const fn new_truncate(addr: u64) -> Self {
        Self(addr % (1 << 52))
    }

    /// Creates a new physical address, without any checks.
    ///
    /// ## Safety
    ///
    /// You must make sure bits 52..64 are zero. This is not checked.
    #[inline]
    pub const unsafe fn new_unsafe(addr: u64) -> Self {
        Self(addr)
    }

    #[inline]
    pub fn align_down<U>(&self, align: U) -> Self
    where
        U: Into<u64>,
    {
        Self(align_down(self.0, align))
    }

    #[inline]
    pub fn align_up<U>(&self, align: U) -> Self
    where
        U: Into<u64>,
    {
        Self(align_up(self.0, align))
    }

    #[inline]
    pub fn is_aligned<U>(self, align: U) -> bool
    where
        U: Into<u64>,
    {
        self.align_down(align) == self
    }

    /// Creates a physical address that points to `0`.
    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Converts the address to an `u64`.
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Convenience method for checking if a physical address is null.
    #[inline]
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
}

/// Represents a physical address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(transparent)]
pub struct VirtAddr(pub u64);

impl VirtAddr {
    /// Creates a instance of `PhysAddr` containing `addr`
    pub fn containing_addr<U: Into<u64>>(addr: u64) -> Self {
        // Align the address down to the closest page
        // start address
        Self(align_down(addr, PAGE_SIZE))
    }

    /// Creates a new physical address, throwing bits 52..64 away.
    #[inline]
    pub const fn new_truncate(addr: u64) -> Self {
        Self(addr % (1 << 52))
    }

    /// Creates a new physical address, without any checks.
    ///
    /// ## Safety
    ///
    /// You must make sure bits 52..64 are zero. This is not checked.
    #[inline]
    pub const unsafe fn new_unsafe(addr: u64) -> Self {
        Self(addr)
    }

    #[inline]
    pub fn align_down<U>(&self, align: U) -> Self
    where
        U: Into<u64>,
    {
        Self(align_down(self.0, align))
    }

    #[inline]
    pub fn align_up<U>(&self, align: U) -> Self
    where
        U: Into<u64>,
    {
        Self(align_up(self.0, align))
    }

    #[inline]
    pub fn is_aligned<U>(self, align: U) -> bool
    where
        U: Into<u64>,
    {
        self.align_down(align) == self
    }

    /// Creates a physical address that points to `0`.
    #[inline]
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Convenience method for checking if a physical address is null.
    #[inline]
    pub const fn is_null(self) -> bool {
        self.0 == 0
    }
}

impl Addr for PhysAddr {
    /// Converts the address to an `u64`.
    #[inline]
    fn as_u64(self) -> u64 {
        self.0
    }
}

impl Addr for VirtAddr {
    /// Converts the address to an `u64`.
    #[inline]
    fn as_u64(self) -> u64 {
        self.0
    }
}

// Operations for Addrs

impl core::ops::Sub for PhysAddr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.as_u64().checked_sub(rhs.as_u64()).unwrap()) // fine to panic on underflows
    }
}

impl core::ops::Sub for VirtAddr {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.as_u64().checked_sub(rhs.as_u64()).unwrap()) // fine to panic on underflows
    }
}

impl core::ops::Add for PhysAddr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.as_u64().checked_add(rhs.as_u64()).unwrap()) // fine to panic on overflows
    }
}

impl core::ops::Add for VirtAddr {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.as_u64().checked_add(rhs.as_u64()).unwrap()) // fine to panic on overflows
    }
}

impl core::ops::Mul for PhysAddr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.as_u64().checked_mul(rhs.as_u64()).unwrap()) // fine to panic on overflows
    }
}

impl core::ops::Mul for VirtAddr {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.as_u64().checked_mul(rhs.as_u64()).unwrap()) // fine to panic on overflows
    }
}

impl core::ops::Add<u64> for PhysAddr {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        Self(self.as_u64().checked_add(rhs).unwrap()) // fine to panic on overflows
    }
}

impl core::ops::Add<u64> for VirtAddr {
    type Output = u64;
    fn add(self, rhs: u64) -> Self::Output {
        self.as_u64().checked_add(rhs).unwrap() // fine to panic on overflows
    }
}

impl core::ops::Sub<u64> for VirtAddr {
    type Output = u64;
    fn sub(self, rhs: u64) -> Self::Output {
        self.as_u64().checked_sub(rhs).unwrap() // fine to panic on underflows
    }
}

impl core::ops::Div<u64> for PhysAddr {
    type Output = Self;
    fn div(self, rhs: u64) -> Self::Output {
        Self(self.as_u64().checked_div(rhs).unwrap()) // fine to panic on overflows
    }
}

impl core::ops::Div<u64> for VirtAddr {
    type Output = Self;
    fn div(self, rhs: u64) -> Self::Output {
        Self(self.as_u64().checked_div(rhs).unwrap()) // fine to panic on overflows
    }
}

impl core::ops::Mul<u64> for PhysAddr {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.as_u64().checked_mul(rhs).unwrap()) // fine to panic on overflows
    }
}

impl core::ops::Mul<u64> for VirtAddr {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        Self(self.as_u64().checked_mul(rhs).unwrap()) // fine to panic on overflows
    }
}

impl Into<u64> for PhysAddr {
    fn into(self) -> u64 {
        self.as_u64()
    }
}

impl Into<u64> for VirtAddr {
    fn into(self) -> u64 {
        self.as_u64()
    }
}
