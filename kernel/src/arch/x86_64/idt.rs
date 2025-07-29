use spin::Mutex;

use crate::{arch::x86_64::DescriptorTablePointer, flag_set, flag_unset};
use core::mem::size_of;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Entry {
    base_low: u16,
    selector: u16,
    ist: u8,
    type_attributes: u8,
    base_middle: u16,
    base_high: u32,
    reserved: u32, // Must be zeroed
}

#[repr(C, packed)]
pub struct InterruptDescriptorTable {
    limit: u16, // sizeof(idt) - 1
    entries: [Entry; 256],
}

const _FLAG_GATE_TRAP: u8 = 0x8E;
const _FLAG_GATE_INT: u8 = 0x8F;

const _FLAG_RING0: u8 = 0 << 5;
const _FLAG_RING1: u8 = 1 << 5;
const _FLAG_RING2: u8 = 2 << 5;
const _FLAG_RING3: u8 = 3 << 5;

const FLAG_PRESENT: u8 = 0x80;

pub fn set_gate(interrupt: usize, base: *const (), selector: u16, flags: u8) {
    let base_addr = base as u64;
    let mut idt = IDT.lock();

    idt.entries[interrupt] = Entry {
        base_low: (base_addr & 0xFFFF) as u16,
        selector,
        ist: 0,
        type_attributes: flags,
        base_middle: ((base_addr >> 16) & 0xFFFF) as u16,
        base_high: (base_addr >> 32) as u32,
        reserved: 0,
    };
}

pub static IDT: Mutex<InterruptDescriptorTable> = Mutex::new(InterruptDescriptorTable {
    limit: (size_of::<[Entry; 256]>() - 1) as u16,
    entries: [Entry {
        base_low: 0,
        selector: 0,
        ist: 0,
        type_attributes: 0,
        base_middle: 0,
        base_high: 0,
        reserved: 0,
    }; 256],
});

pub fn init() {
    let idt = IDT.lock();
    load(&*idt);
    crate::trace!("Initialized IDT");
}

pub fn load(idt: &InterruptDescriptorTable) {
    let idt_ptr: DescriptorTablePointer = DescriptorTablePointer {
        limit: idt.limit,
        base: idt.entries.as_ptr() as u64,
    };

    unsafe {
        core::arch::asm!(
            "lidt [{0}]",
            in(reg) &idt_ptr,
            options(nostack, preserves_flags),
        );
    }
}

pub fn enable_gate(interrupt: usize) {
    let mut idt = IDT.lock();
    flag_unset!(idt.entries[interrupt].type_attributes, FLAG_PRESENT);
}

pub fn disable_gate(interrupt: usize) {
    let mut idt = IDT.lock();
    flag_set!(idt.entries[interrupt].type_attributes, FLAG_PRESENT);
}
