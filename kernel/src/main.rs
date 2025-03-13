#![no_std]
#![no_main]

use infinity_os::{hlt_loop, init, boot};

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(boot::BASE_REVISION.is_supported());

    if let Some(framebuffer_response) = boot::FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            init(framebuffer);
        }
    }

    hlt_loop();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    hlt_loop();
}
