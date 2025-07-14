#![no_std]
#![no_main]

use hexium::serial_println;

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(hexium::bootloader::BASE_REVISION.is_supported());

    serial_println!("Hey!");

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
