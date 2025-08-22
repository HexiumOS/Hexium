use core::{ffi::c_void, ptr, slice};

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

pub fn create_slice<'a, T>(addr: u64, len: usize) -> &'a [T] {
    let ptr = addr as *const T;
    unsafe { slice::from_raw_parts(ptr, len) }
}

pub fn create_slice_mut<'a, T>(addr: u64, len: usize) -> &'a mut [T] {
    let ptr = addr as *mut T;
    unsafe { slice::from_raw_parts_mut(ptr, len) }
}

#[macro_export]
macro_rules! flag_set {
    ($x:expr, $flag:expr) => {
        $x |= $flag;
    };
}

#[macro_export]
macro_rules! flag_unset {
    ($x:expr, $flag:expr) => {
        $x &= !$flag;
    };
}
