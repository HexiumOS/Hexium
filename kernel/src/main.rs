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
    use hexium_os::utils::registers::*;
    panic_log!("{}\n", info);
    print_register_dump(&get_registers());
    hlt_loop();
}
