#![allow(dead_code)]

use core::arch::asm;
use core::mem::size_of;
use spin::Mutex;

#[repr(C, packed)]
struct Entry {
    limit_low: u16,       // bits 0–15 of limit
    base_low: u16,        // bits 0–15 of base
    base_middle: u8,      // bits 16–23 of base
    access: u8,           // access flags
    flags_limit_high: u8, // bits 16–19 of limit + flags (packed)
    base_high: u8,        // bits 24–31 of base
}

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    limit: u16, // sizeof(gdt) - 1
    entries: [Entry; 3],
}

// Access flags defined as consts to ensure const evaluation
const ACCESS_CODE_READABLE: u8 = 0x02;
const ACCESS_DATA_WRITEABLE: u8 = 0x02;
const ACCESS_CODE_CONFORMING: u8 = 0x04;
const ACCESS_DATA_DIRECTION_NORMAL: u8 = 0x00;
const ACCESS_DATA_DIRECTION_DOWN: u8 = 0x04;
const ACCESS_DATA_SEGMENT: u8 = 0x10;
const ACCESS_CODE_SEGMENT: u8 = 0x18;
const ACCESS_DESCRIPTOR_TSS: u8 = 0x00;
const ACCESS_RING0: u8 = 0x00;
const ACCESS_RING1: u8 = 0x20;
const ACCESS_RING2: u8 = 0x40;
const ACCESS_RING3: u8 = 0x60;
const ACCESS_PRESENT: u8 = 0x80;

// Flags defined as consts to ensure const evaluation
const FLAGS_BIT64: u8 = 0x20;
const FLAGS_BIT32: u8 = 0x40;
const FLAGS_BIT16: u8 = 0x00;
const FLAGS_GRANULARITY_1B: u8 = 0x00;
const FLAGS_GRANULARITY_4K: u8 = 0x80;

// Helper functions
const fn gdt_limit_low(limit: u32) -> u16 {
    (limit & 0xFFFF) as u16
}

const fn gdt_base_low(base: u32) -> u16 {
    (base & 0xFFFF) as u16
}

const fn gdt_base_middle(base: u32) -> u8 {
    ((base >> 16) & 0xFF) as u8
}

const fn gdt_flags_limit_high(limit: u32, flags: u8) -> u8 {
    ((((limit >> 16) & 0xF) as u8) | (flags & 0xF0)) as u8
}

const fn gdt_base_high(base: u32) -> u8 {
    ((base >> 24) & 0xFF) as u8
}

macro_rules! gdt_entry {
    ($base:expr, $limit:expr, $access:expr, $flags:expr) => {
        Entry {
            limit_low: gdt_limit_low($limit),
            base_low: gdt_base_low($base),
            base_middle: gdt_base_middle($base),
            access: $access,
            flags_limit_high: gdt_flags_limit_high($limit, $flags),
            base_high: gdt_base_high($base),
        }
    };
}

pub static GDT: Mutex<GlobalDescriptorTable> = Mutex::new(GlobalDescriptorTable {
    limit: (size_of::<[Entry; 3]>() - 1) as u16,
    entries: [
        // Null descriptor
        gdt_entry!(0, 0, 0, 0),
        // Kernel 64-bit code segment
        gdt_entry!(
            0,
            0xFFFF,
            ACCESS_PRESENT | ACCESS_RING0 | ACCESS_CODE_SEGMENT | ACCESS_CODE_READABLE,
            FLAGS_BIT64 | FLAGS_GRANULARITY_4K
        ),
        // Kernel 64-bit data segment
        gdt_entry!(
            0,
            0xFFFF,
            ACCESS_PRESENT | ACCESS_RING0 | ACCESS_DATA_SEGMENT | ACCESS_DATA_WRITEABLE,
            FLAGS_GRANULARITY_4K
        ),
    ],
});

pub fn init() {
    let gdt = GDT.lock();
    load(&*gdt, 0x08, 0x10);
}

// Loads the GDT and update the code and data segments
pub fn load(gdt: &GlobalDescriptorTable, cs: u16, ds: u16) {
    #[repr(C, packed)]
    struct DescriptorTablePointer {
        limit: u16,
        base: u64,
    }

    let gdt_ptr = DescriptorTablePointer {
        limit: gdt.limit,
        base: gdt.entries.as_ptr() as u64,
    };

    unsafe {
        asm!(
            "lgdt [{0}]",
            in(reg) &gdt_ptr,
            options(nostack, preserves_flags),
        );
    }
    crate::debug!("Loaded GDT");

    unsafe {
        asm!(
            "mov ds, {0:e}",
            "mov es, {0:e}",
            "mov fs, {0:e}",
            "mov gs, {0:e}",
            "mov ss, {0:e}",
            "push {1:r}",               // push CS
            "lea {2:r}, [rip + 2f]",
            "push {2:r}",               // push return address
            "retfq",                    // far return
            "2:",
            in(reg) ds,
            in(reg) cs,
            lateout(reg) _,
            options(preserves_flags),
        );
    }

    crate::debug!("Updated segments");
}
