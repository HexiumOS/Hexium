#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use core::arch::asm;

pub mod boot;
pub mod devices;
pub mod drivers;
pub mod fs;
pub mod interrupts;
pub mod log;
pub mod memory;
pub mod rtc;
pub mod utils;
pub mod writer;

pub fn init() {
    writer::init();
    interrupts::init();
    memory::init();
    fs::ramfs::init();

    use crate::fs::memfs::MemFS;
    use crate::fs::vfs::FileType;
    use crate::fs::vfs::FileSystem;

    let mut memfs = MemFS::new();
    memfs.mount("/").unwrap();
    
    let file = memfs.create("/test.txt", FileType::File).unwrap();
    memfs.write(&file, b"Hello, World!", 0).unwrap();
    
    let mut buffer = [0u8; 13];
    memfs.read(&file, &mut buffer, 0).unwrap();
    
    assert_eq!(&buffer, b"Hello, World!");
    println!("Read content: {}", core::str::from_utf8(&buffer).unwrap());

    info!(
        "Hexium OS kernel v{} succesfully initialized at {}\n",
        env!("CARGO_PKG_VERSION"),
        unsafe { rtc::read_rtc() }
    );
    info!("Welcome to Hexium OS\n");
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
