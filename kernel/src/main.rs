#![no_std]
#![no_main]

use hexium::{debug, panic_log};

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    hexium::hal::init();
    hexium::info!("Welcome to HexiumOS");
    hexium::hal::halt_device();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic_log!("----- KERNEL PANIC -----");
    panic_log!("{}", info);
    hexium::hal::halt_device();
}
