#![no_std]
#![no_main]

use core::arch::asm;

pub mod bootloader;

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(bootloader::BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = bootloader::FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            for i in 0..100_u64 {
                let pixel_offset = i * framebuffer.pitch() + i * 4;

                unsafe {
                    framebuffer
                        .addr()
                        .add(pixel_offset as usize)
                        .cast::<u32>()
                        .write(0xFFFFFFFF)
                }
            }
        }
    }

    halt_device();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    halt_device();
}

fn halt_device() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}
