pub mod boot;
pub mod gdt;
pub mod writer;

pub fn init() {
    assert!(boot::BASE_REVISION.is_supported());
    writer::init();
}
