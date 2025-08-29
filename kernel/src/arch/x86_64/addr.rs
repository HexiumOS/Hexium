use core::fmt;

#[repr(transparent)]
pub struct VirtAddr(u64);

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct PhysAddr(u64);

pub struct VirtAddrNotValid(pub u64);
pub struct PhysAddrNotValid(pub u64);

impl VirtAddr {
    #[inline]
    pub const fn new(addr: u64) -> VirtAddr {
        match Self::try_new(addr) {
            Ok(v) => v,
            Err(_) => panic!("Virtual address must be sign extended in bits 48 to 64"),
        }
    }

    #[inline]
    pub const fn try_new(addr: u64) -> Result<VirtAddr, VirtAddrNotValid> {
        let v = Self::new_truncate(addr);
        if v.0 == addr {
            Ok(v)
        } else {
            Err(VirtAddrNotValid(addr))
        }
    }

    #[inline]
    pub const fn new_truncate(addr: u64) -> VirtAddr {
        VirtAddr(((addr << 16) as i64 >> 16) as u64)
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    #[inline]
    pub fn from_ptr<T: ?Sized>(ptr: *const T) -> Self {
        Self::new(ptr as *const () as u64)
    }

    #[inline]
    pub const fn as_ptr<T>(self) -> *const T {
        self.as_u64() as *const T
    }

    #[inline]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.as_ptr::<T>() as *mut T
    }
}

impl fmt::Pointer for VirtAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&(self.0 as *const ()), f)
    }
}

impl PhysAddr {
    #[inline]
    pub const fn new(addr: u64) -> Self {
        // TODO: Replace with .ok().expect(msg) when that works on stable.
        match Self::try_new(addr) {
            Ok(p) => p,
            Err(_) => panic!("physical addresses must not have any bits in the range 52 to 64 set"),
        }
    }

    #[inline]
    pub const fn try_new(addr: u64) -> Result<Self, PhysAddrNotValid> {
        let p = Self::new_truncate(addr);
        if p.0 == addr {
            Ok(p)
        } else {
            Err(PhysAddrNotValid(addr))
        }
    }

    #[inline]
    pub const fn new_truncate(addr: u64) -> PhysAddr {
        PhysAddr(addr % (1 << 52))
    }

    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }
}
