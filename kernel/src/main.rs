#![no_std]
#![no_main]

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    hexium::hal::init();
    hexium::info!("Welcome to HexiumOS");
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };
    hexium::hal::halt_device();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    hexium::hal::halt_device();
}
