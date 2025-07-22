#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct VirtAddr(u64);

impl VirtAddr {
    #[inline]
    pub const fn new(addr: u64) -> VirtAddr {
        match Self::try_new(addr) {
            Ok(v) => v,
            Err(_) => panic!("virtual address must be sign extended in bits 48 to 64"),
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
}

pub struct VirtAddrNotValid(pub u64);
