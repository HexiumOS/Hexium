#![no_std]
#![no_main]

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    hexium::arch::init();
    halt_device();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    halt_device();
}

fn halt_device() -> ! {
    hexium::arch::interrupts::disable();
    loop {
        hexium::arch::interrupts::wait();
    }
}
