#![no_std]
#![no_main]

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    halt_device();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    halt_device();
}

fn halt_device() -> ! {
    unsafe {
        core::arch::asm!("cli");
    }
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}
