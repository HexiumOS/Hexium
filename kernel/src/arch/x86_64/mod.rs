pub mod addr;
pub mod boot;
pub mod drivers;
pub mod gdt;
pub mod idt;
pub mod instructions;
pub mod interrupts;
pub mod registers;
pub mod tss;
pub mod writer;

pub fn init() {
    interrupts::disable();
    assert!(boot::BASE_REVISION.is_supported());

    drivers::uart_16650::init();
    writer::init();
    gdt::init();
    tss::init();
    idt::init();
    drivers::pic8259::init();
    interrupts::enable();
}

#[repr(u8)]
pub enum PrivilegeLevel {
    Ring0 = 0,
    Ring1 = 1,
    Ring2 = 2,
    Ring3 = 3,
}

impl PrivilegeLevel {
    #[inline]
    pub const fn from_u16(value: u16) -> PrivilegeLevel {
        match value {
            0 => PrivilegeLevel::Ring0,
            1 => PrivilegeLevel::Ring1,
            2 => PrivilegeLevel::Ring2,
            3 => PrivilegeLevel::Ring3,
            _ => panic!("invalid privilege level"),
        }
    }
}

#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
    pub limit: u16,
    pub base: addr::VirtAddr,
}
