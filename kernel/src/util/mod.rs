use core::{ffi::c_void, ptr};

#[macro_export]
macro_rules! retry_until_ok {
    ($cond:expr) => {
        loop {
            if let Ok(ok) = $cond {
                break ok;
            }
            core::hint::spin_loop();
        }
    };
}

pub fn option_to_c_void<T>(opt: Option<&mut T>) -> *mut c_void {
    opt.map_or(ptr::null_mut(), |reference| {
        reference as *mut T as *mut c_void
    })
}
