#![no_std]
#![no_main]

use hexium_os::{boot, hlt_loop, init, print};

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(boot::BASE_REVISION.is_supported());

    init();

    hlt_loop();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    use hexium_os::utils::registers::*;
    print!("\n\n");
    let location = info.location().unwrap();

    print!("Message: {}\n\n", info.message());
    print!(
        "Location: {}:{}:{}\n\n",
        location.file(),
        location.line(),
        location.column()
    );
    print!("Register dump:\n");
    print_register_dump(&get_registers());
    print!("Stack trace:\nUnknown\n\n");
    print!("System halted.\n\n");

    hlt_loop();
}
