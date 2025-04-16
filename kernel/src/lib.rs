#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]

extern crate alloc;

use alloc::string::String;
use core::{arch::asm, panic::PanicInfo};
use utils::registers::{get_registers, print_register_dump};

pub mod boot;
pub mod devices;
pub mod drivers;
pub mod fs;
pub mod interrupts;
pub mod log;
pub mod memory;
pub mod rtc;
pub mod serial;
pub mod task;
pub mod utils;
pub mod writer;
pub mod tests;

pub fn init() {
    writer::init();
    interrupts::init();
    memory::init();

    let mut vfs = fs::vfs::VFS::new(None);
    fs::ramfs::init(&mut vfs);
    
    info!("Before");
    print_startup_message(&mut vfs);
    info!("After");

    // RYAN_NOTES: Commented out for now as the code doesn't run past this section. Will return it back.
    // let mut executor = crate::task::executor::Executor::new();
    // let _ = executor.spawn(crate::task::Task::new(devices::keyboard::trace_keypresses()));
    // executor.run();

    info!("After2");
    //vfs.unmount_fs();
}

fn print_startup_message(vfs: &mut fs::vfs::VFS) -> [u8; 128] {
    let mut buffer = [0u8; 128];

    match vfs.open_file("./welcome.txt") {
        Ok(vnode) => match vfs.read_file(&vnode, &mut buffer, 0) {
            Ok(_bytes_read) => {}
            Err(err) => {
                error!("Error reading file: {}", err);
            }
        },
        Err(err) => {
            error!("File not found: {}", err);
        }
    }

    info!("Testing");

    info!(
        "Hexium OS kernel v{} succesfully initialized at {}",
        env!("CARGO_PKG_VERSION"),
        unsafe { rtc::read_rtc() }
    );
    info!("{}", String::from_utf8_lossy(&buffer));

    buffer
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

#[cfg(test)]
#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn rust_panic(info: &core::panic::PanicInfo) -> ! {
    panic_log!("{}\n", info);
    print_register_dump(&get_registers());
    #[cfg(test)]
    println!("Test");
    #[cfg(not(test))]
    println!("Not Test");
    print!("Main Panic");
    hlt_loop();
}

// #[cfg(not(test))]
// #[panic_handler]
// fn rust_panic(info: &core::panic::PanicInfo) -> ! {
//     serial_println!("Rust panic");
//     #[cfg(test)]
//     test_panic_handler(&info);    
//     #[cfg(not(test))]
//     main_panic_handler(&info);
//     hlt_loop();
// }

// fn main_panic_handler(info: &PanicInfo) {
//     println!("Main panic handler");
//     panic_log!("{}\n", info);
//     print_register_dump(&get_registers());
//     #[cfg(test)]
//     println!("Test");
//     #[cfg(not(test))]
//     println!("Not Test");
//     print!("Main Panic");
// }

// fn test_panic_handler(info: &PanicInfo) {
//     println!("Test panic handler");
//     serial_println!("[failed]");
//     serial_println!("Error: {}", info);
//     exit_qemu(QemuExitCode::Failed);
// }

fn test_runner(_test: &[&i32]) {
    loop {}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }
