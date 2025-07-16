use crate::arch::PrivilegeLevel;
use bit_field::BitField;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    /// NOTE: index is not the offset
    #[inline]
    pub const fn new(index: u16, rpl: PrivilegeLevel) -> SegmentSelector {
        SegmentSelector((index << 3) | (rpl as u16))
    }

    /// Can be used as a selector into a non-existent segment and assigned to segment registers,
    /// e.g. data segment register in ring 0
    pub const NULL: Self = Self::new(0, PrivilegeLevel::Ring0);

    /// GDT index.
    #[inline]
    pub fn index(self) -> u16 {
        self.0 >> 3
    }

    #[inline]
    pub fn rpl(self) -> PrivilegeLevel {
        PrivilegeLevel::from_u16(self.0.get_bits(0..2))
    }

    /// Set the privilege level for this selector.
    #[inline]
    pub fn set_rpl(&mut self, rpl: PrivilegeLevel) {
        self.0.set_bits(0..2, rpl as u16);
    }
}

pub trait Segment {
    fn get_reg() -> SegmentSelector;
    unsafe fn set_reg(sel: SegmentSelector);
}

/// While most fields in the Code-Segment are unused in 64-bit
/// long mode, some of them must be set to a specific value. The
/// EXECUTABLE, USER_SEGMENT, and LONG_MODE  bits must be set, while the
/// DEFAULT_SIZE bit must be unset.
///
/// The DPL_RING_3 field can be used to change privilege level.
///
/// All other fields (like the segment base and limit) are ignored by the
/// processor and setting them has no effect.
#[derive(Debug)]
pub struct CS;

/// Entirely unused in 64-bit mode; setting the segment register does nothing.
/// However, in ring 3, the SS register still has to point to a valid
/// Descriptor. This means a user-mode read/write segment descriptor must be
/// present in the GDT.
///
/// This register is also set by the `syscall`/`sysret` and
/// `sysenter`/`sysexit` instructions (even on 64-bit transitions). This is to
/// maintain symmetry with 32-bit transitions where setting SS actually will
/// actually have an effect.
#[derive(Debug)]
pub struct SS;

/// Data segment is unused in long mode
#[derive(Debug)]
pub struct DS;

/// Entirely unused in long mode
#[derive(Debug)]
pub struct ES;

/// Only base is used in 64-bit mode, see Segment64. This is often used in
/// user-mode for Thread-Local Storage (TLS).
#[derive(Debug)]
pub struct FS;

/// In kernel-mode, the GS base often points to a per-cpu kernel data structure.
#[derive(Debug)]
pub struct GS;

macro_rules! get_reg_impl {
    ($name:literal) => {
        #[inline]
        fn get_reg() -> SegmentSelector {
            let segment: u16;
            unsafe {
                core::arch::asm!(concat!("mov {0:x}, ", $name), out(reg) segment, options(nomem, nostack, preserves_flags));
            }
            SegmentSelector(segment)
        }
    };
}

macro_rules! segment_impl {
    ($type:ty, $name:literal) => {
        impl Segment for $type {
            get_reg_impl!($name);

            #[inline]
            unsafe fn set_reg(sel: SegmentSelector) {
                unsafe {
                    core::arch::asm!(concat!("mov ", $name, ", {0:x}"), in(reg) sel.0, options(nostack, preserves_flags));
                }
            }
        }
    };
}

impl Segment for CS {
    get_reg_impl!("cs");

    #[inline]
    unsafe fn set_reg(sel: SegmentSelector) {
        unsafe {
            core::arch::asm!(
                "push {sel}",
                "lea {tmp}, [55f + rip]",
                "push {tmp}",
                "retfq",
                "55:",
                sel = in(reg) u64::from(sel.0),
                tmp = lateout(reg) _,
                options(preserves_flags),
            );
        }
    }
}

segment_impl!(SS, "ss");
segment_impl!(DS, "ds");
segment_impl!(ES, "es");
segment_impl!(FS, "fs");
segment_impl!(GS, "gs");
