use crate::arch::DescriptorTablePointer;
use crate::arch::addr::VirtAddr;
use crate::arch::registers::segmentation::{CS, SS, SegmentSelector};
use crate::arch::x86_64::registers::segmentation::Segment;
use crate::debug;
use crate::{arch::PrivilegeLevel, trace};

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        SS::set_reg(GDT.1.data_selector);
    }

    GDT.0.verify();

    trace!("Initialized GDT");
}

lazy_static::lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let data_selector = gdt.append(Descriptor::kernel_data_segment());
        (gdt, Selectors {
            code_selector,
            data_selector,
        })
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
}

// A GDT Entry is 8 bytes so it can be represented using a transparent u64
#[repr(transparent)]
pub struct Entry(u64);

impl Entry {
    const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

pub struct GlobalDescriptorTable<const MAX: usize = 8> {
    table: [Entry; MAX],
    len: usize,
}

impl GlobalDescriptorTable {
    pub const fn new() -> Self {
        Self::empty()
    }
}

impl<const MAX: usize> GlobalDescriptorTable<MAX> {
    #[inline]
    pub const fn empty() -> Self {
        assert!(MAX > 0, "A GDT cannot have 0 entries");
        assert!(MAX <= (1 << 13), "A GDT can only have at most 2^13 entries");

        const NULL: Entry = Entry::new(0);
        Self {
            table: [NULL; MAX],
            len: 1,
        }
    }

    #[inline]
    pub const fn append(&mut self, entry: Descriptor) -> SegmentSelector {
        let index = match entry {
            Descriptor::UserSegment(value) => {
                if self.len > self.table.len().saturating_sub(1) {
                    panic!("GDT full");
                }
                self.push(value)
            }
            Descriptor::SystemSegment(value_low, value_high) => {
                if self.len > self.table.len().saturating_sub(2) {
                    panic!("GDT requires two free spaces to hold a SystemSegment");
                }
                let index = self.push(value_low);
                self.push(value_high);
                index
            }
        };
        SegmentSelector::new(index as u16, entry.dpl())
    }

    #[inline]
    const fn push(&mut self, value: u64) -> usize {
        let index = self.len;
        self.table[index] = Entry::new(value);
        self.len += 1;
        index
    }

    fn pointer(&self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            limit: self.limit(),
            base: VirtAddr::new(self.table.as_ptr() as u64),
        }
    }

    pub const fn limit(&self) -> u16 {
        use core::mem::size_of;
        // 0 < self.next_free <= MAX <= 2^13, so the limit calculation
        // will not underflow or overflow.
        (self.len * size_of::<u64>() - 1) as u16
    }

    #[inline]
    pub fn load(&'static self) {
        trace!("Loading GDT...");
        unsafe { self.unsafe_load() };
    }

    #[inline]
    pub unsafe fn unsafe_load(&self) {
        unsafe {
            lgdt(&self.pointer());
        }
    }

    pub fn verify(&self) {
        trace!("Verifying GDT load...");
        let gdtr = read_gdtr();
        let expected = GDT.0.pointer();

        debug!(
            "GDTR: base={:#018x}, limit={:#06x}",
            gdtr.base.as_u64(),
            gdtr.limit
        );
        debug!(
            "Expected: base={:#018x}, limit={:#06x}",
            expected.base.as_u64(),
            expected.limit
        );

        if gdtr.base.as_u64() != expected.base.as_u64() || gdtr.limit != expected.limit {
            panic!(
                "GDT mismatch!\nLoaded GDTR: base={:#018x}, limit={:#06x}\nExpected:    base={:#018x}, limit={:#06x}",
                gdtr.base.as_u64(),
                gdtr.limit,
                expected.base.as_u64(),
                expected.limit
            );
        }

        let cs: u16;
        let ss: u16;
        unsafe {
            core::arch::asm!("mov {0:x}, cs", out(reg) cs);
            core::arch::asm!("mov {0:x}, ss", out(reg) ss);
        }

        debug!(
            "CS: {:#04x} (expected {:#04x}), SS: {:#04x} (expected {:#04x})",
            cs, GDT.1.code_selector.0, ss, GDT.1.data_selector.0
        );

        assert_eq!(
            cs, GDT.1.code_selector.0,
            "CS does not match GDT code selector"
        );
        assert_eq!(
            ss, GDT.1.data_selector.0,
            "SS does not match GDT data selector"
        );

        trace!("GDT successfully verified.");
    }
}

impl Default for GlobalDescriptorTable {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub enum Descriptor {
    UserSegment(u64),
    SystemSegment(u64, u64),
}

impl Descriptor {
    #[inline]
    pub const fn dpl(self) -> super::PrivilegeLevel {
        let value_low = match self {
            Descriptor::UserSegment(v) => v,
            Descriptor::SystemSegment(v, _) => v,
        };
        let dpl = (value_low & DescriptorFlags::DPL_RING_3.bits()) >> 45;
        PrivilegeLevel::from_u16(dpl as u16)
    }

    #[inline]
    pub const fn kernel_code_segment() -> Descriptor {
        Descriptor::UserSegment(DescriptorFlags::KERNEL_CODE64.bits())
    }

    #[inline]
    pub const fn kernel_data_segment() -> Descriptor {
        Descriptor::UserSegment(DescriptorFlags::KERNEL_DATA.bits())
    }

    #[inline]
    pub const fn user_data_segment() -> Descriptor {
        Descriptor::UserSegment(DescriptorFlags::USER_DATA.bits())
    }

    #[inline]
    pub const fn user_code_segment() -> Descriptor {
        Descriptor::UserSegment(DescriptorFlags::USER_CODE64.bits())
    }
}

#[inline]
pub unsafe fn lgdt(gdt: &DescriptorTablePointer) {
    unsafe {
        core::arch::asm!("lgdt [{}]", in(reg) gdt, options(readonly, nostack, preserves_flags));
    }
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
    pub struct DescriptorFlags: u64 {
        const ACCESSED          = 1 << 40;
        const WRITABLE          = 1 << 41;
        const CONFORMING        = 1 << 42;
        const EXECUTABLE        = 1 << 43;
        const USER_SEGMENT      = 1 << 44;
        const DPL_RING_3        = 3 << 45;
        const PRESENT           = 1 << 47;
        const AVAILABLE         = 1 << 52;
        const LONG_MODE         = 1 << 53;
        const DEFAULT_SIZE      = 1 << 54;
        const GRANULARITY       = 1 << 55;
        const LIMIT_0_15        = 0xFFFF;
        const LIMIT_16_19       = 0xF << 48;
        const BASE_0_23         = 0xFF_FFFF << 16;
        const BASE_24_31        = 0xFF << 56;
    }
}

impl DescriptorFlags {
    const COMMON: Self = Self::from_bits_truncate(
        Self::USER_SEGMENT.bits()
            | Self::PRESENT.bits()
            | Self::WRITABLE.bits()
            | Self::ACCESSED.bits()
            | Self::LIMIT_0_15.bits()
            | Self::LIMIT_16_19.bits()
            | Self::GRANULARITY.bits(),
    );
    pub const KERNEL_DATA: Self =
        Self::from_bits_truncate(Self::COMMON.bits() | Self::DEFAULT_SIZE.bits());
    pub const KERNEL_CODE64: Self = Self::from_bits_truncate(
        Self::COMMON.bits() | Self::EXECUTABLE.bits() | Self::LONG_MODE.bits(),
    );
    pub const USER_DATA: Self =
        Self::from_bits_truncate(Self::KERNEL_DATA.bits() | Self::DPL_RING_3.bits());
    pub const USER_CODE64: Self =
        Self::from_bits_truncate(Self::KERNEL_CODE64.bits() | Self::DPL_RING_3.bits());
}

fn read_gdtr() -> DescriptorTablePointer {
    let mut gdtr = DescriptorTablePointer {
        limit: 0,
        base: VirtAddr::new(0),
    };

    unsafe {
        core::arch::asm!(
            "sgdt [{}]",
            in(reg) &mut gdtr,
            options(nostack, preserves_flags)
        );
    }

    gdtr
}
