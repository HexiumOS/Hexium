pub mod boot;
pub mod drivers;
pub mod interrupts;
pub mod registers;

pub fn init() {
    assert!(boot::BASE_REVISION.is_supported());

    drivers::uart_16650::init();
}
