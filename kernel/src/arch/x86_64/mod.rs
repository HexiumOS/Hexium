pub mod boot;

pub fn init() {
    assert!(boot::BASE_REVISION.is_supported());
}
