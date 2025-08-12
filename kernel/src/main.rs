#![no_std]
#![no_main]

unsafe extern "C" {
    fn super_cool_symbol() -> u64;
}

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    hexium::hal::init();
    hexium::info!("Welcome to HexiumOS");
    //unsafe {
    //    let result = super_cool_symbol();
    //    hexium::println!("Result from super_cool_symbol: {}", result);
    //}
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };
    hexium::hal::halt();
}

#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    hexium::println!("{}", info);
    hexium::hal::halt();
}
