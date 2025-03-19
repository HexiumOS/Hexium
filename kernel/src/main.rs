#![no_std]
#![no_main]

use hexium_os::{boot, hlt_loop, init, panic_log};

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(boot::BASE_REVISION.is_supported());

    init();

    hlt_loop();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    panic_log!("{}", info);
    hlt_loop();
}
