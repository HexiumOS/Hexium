#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use alloc::string::String;
use core::arch::asm;

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

pub fn init() {
    writer::init();
    interrupts::init();
    memory::init();

    let mut vfs = fs::vfs::VFS::new(None);
    fs::ramfs::init(&mut vfs);

    print_startup_message(&mut vfs);
    //vfs.unmount_fs();
}

fn print_startup_message(vfs: &mut fs::vfs::VFS) -> [u8; 128] {
    let mut buffer = [0u8; 128];

    match vfs.open_file("./welcome.txt") {
        Ok(vnode) => match vfs.read_file(&vnode, &mut buffer, 0) {
            Ok(_bytes_read) => {}
            Err(err) => {
                error!("Error reading file: {}\n", err);
            }
        },
        Err(err) => {
            error!("File not found: {}\n", err);
        }
    }

    info!(
        "Hexium OS kernel v{} succesfully initialized at {}\n",
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
