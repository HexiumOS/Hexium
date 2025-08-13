#![no_std]
#![no_main]
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    hexium::hal::init();
    hexium::info!("Welcome to HexiumOS");
    hexium::hal::halt();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    hexium::hal::panic::kpanic(info.message().as_str().unwrap_or_default(), 0, None);
}
