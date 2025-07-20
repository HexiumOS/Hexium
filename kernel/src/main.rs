#![no_std]
#![no_main]

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    hexium::hal::init();
    hexium::hal::halt_device();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    hexium::hal::halt_device();
}
