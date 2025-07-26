#![no_std]
#![no_main]

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    hexium::hal::init();
    hexium::hal::halt();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hexium::hal::halt();
}
