pub mod boot;
pub mod gdt;
pub mod idt;
pub mod tss;
pub mod writer;

pub fn init() {
    assert!(boot::BASE_REVISION.is_supported());
    writer::init();
    gdt::init();
    tss::init();
    idt::init();
}
