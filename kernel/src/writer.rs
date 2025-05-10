/*
 * This file is part of Hexium OS.
 * Copyright (C) 2025 The Hexium OS Authors – see the AUTHORS file.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{boot, utils::types::option_to_c_void};
use core::fmt;
use core::ptr;
use lazy_static::lazy_static;
use spin::Mutex;

pub fn init() {
    if let Some(framebuffer_response) = boot::FRAMEBUFFER_REQUEST.get_response() {
        if let Some(framebuffer) = framebuffer_response.framebuffers().next() {
            *FLANTERM_CTX.lock() = FlantermContextWrapper::new(unsafe {
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
}

pub struct FlantermContextWrapper(*mut flanterm::sys::flanterm_context);

impl FlantermContextWrapper {
    pub fn new(context: *mut flanterm::sys::flanterm_context) -> Self {
        FlantermContextWrapper(context)
    }

    pub fn inner(&self) -> *mut flanterm::sys::flanterm_context {
        self.0
    }
}

unsafe impl Send for FlantermContextWrapper {}
unsafe impl Sync for FlantermContextWrapper {}

lazy_static! {
    pub static ref FLANTERM_CTX: Mutex<FlantermContextWrapper> =
        Mutex::new(FlantermContextWrapper(ptr::null_mut()));
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {});
}

pub struct Writer {}

impl Writer {
    fn write_string(&mut self, s: &str) {
        unsafe {
            flanterm::sys::flanterm_write(
                FLANTERM_CTX.lock().inner(),
                s.as_ptr() as *const i8,
                s.len(),
            )
        };
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        $crate::writer::_print(format_args!($($arg)*));
        $crate::serial_print!("{}", format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
