#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::string::String;
use core::{arch::asm, panic::PanicInfo};

pub mod arch;
pub mod boot;
pub mod debug;
pub mod devices;
pub mod drivers;
pub mod fs;
pub mod hal;
pub mod log;
pub mod rsod;
pub mod rtc;
pub mod task;
pub mod utils;
pub mod writer;

pub fn init() {
    hal::init();

    let mut vfs = hal::vfs::Vfs::new();
    fs::ramfs::init(&vfs);

    print_startup_message(&mut vfs);

    // Issue#30: Commented out for now as the code doesn't run past this section. Will return it back.
    //{
    //    let mut executor = crate::task::executor::Executor::new();
    //    let _ = executor.spawn(crate::task::Task::new(devices::keyboard::trace_keypresses()));
    //    executor.run();
    //}
}

fn print_startup_message(vfs: &hal::vfs::Vfs) {
    let file: hal::vfs::Vnode = match vfs.lookuppn("/ramdisk/welcome.txt") {
        Ok(file) => file,
        Err(err) => {
            error!("File lookup error for 'ramdisk/welcome.txt': {:?}", err);
            return;
        }
    };

    let mut buffer = [0u8; 64];

    match file.ops.read(&file, &mut buffer, 0, 64) {
        Ok(_) => {}
        Err(err) => {
            error!("File read error for 'ramdisk/welcome.txt': {:?}", err);
        }
    }

    info!(
        "Hexium OS kernel v{} successfully initialized at {}",
        env!("CARGO_PKG_VERSION"),
        unsafe { rtc::read_rtc() }
    );
    info!("{}", String::from_utf8_lossy(&buffer));
}

pub fn hlt_loop() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
            #[cfg(any(target_arch = "aarch64", target_arch = "riscv64"))]
            asm!("wfi");
            #[cfg(target_arch = "loongarch64")]
            asm!("idle 0");
        }
    }
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());

    for test in tests {
        test.run();
    }

    exit_qemu(QemuExitCode::Success);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64c::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub trait Testable {
    fn run(&self);
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

#[cfg(test)]
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    test_main();
    loop {}
}

#[cfg(test)]
#[panic_handler]
/// Handles panics during library-specific tests
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
