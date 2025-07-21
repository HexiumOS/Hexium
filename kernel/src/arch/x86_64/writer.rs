use crate::utils::option_to_c_void;
use core::{fmt, ptr};
use spin::Mutex;

pub struct FlantermContext(pub *mut flanterm::sys::flanterm_context);
unsafe impl Send for FlantermContext {}
unsafe impl Sync for FlantermContext {}

pub fn init() {
    if let Some(framebuffer_response) = super::boot::FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            *FLANTERM_CTX.lock() = FlantermContext(unsafe {
                flanterm::sys::flanterm_fb_init(
                    None,
                    None,
                    framebuffer.addr() as *mut u32,
                    framebuffer.width() as usize,
                    framebuffer.height() as usize,
                    framebuffer.pitch() as usize,
                    framebuffer.red_mask_size(),
                    framebuffer.red_mask_shift(),
                    framebuffer.green_mask_size(),
                    framebuffer.green_mask_shift(),
                    framebuffer.blue_mask_size(),
                    framebuffer.blue_mask_shift(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    option_to_c_void::<fn()>(None),
                    0,
                    0,
                    1,
                    None::<fn()>.is_some() as usize,
                    None::<fn()>.is_some() as usize,
                    None::<fn()>.is_some() as usize,
                )
            });
        }
    }

    crate::trace!("Initialized Flanterm context");
}

lazy_static::lazy_static! {
    pub static ref FLANTERM_CTX: Mutex<FlantermContext> =
        Mutex::new(FlantermContext(ptr::null_mut()));
    pub static ref WRITER: Mutex<FlantermWriter> = Mutex::new(FlantermWriter {});
}

pub struct FlantermWriter {}

impl FlantermWriter {
    fn write_string(&mut self, s: &str) {
        unsafe {
            flanterm::sys::flanterm_write(FLANTERM_CTX.lock().0, s.as_ptr() as *const i8, s.len())
        };
    }
}

impl fmt::Write for FlantermWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use crate::arch::interrupts;
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
