#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use hexium_os::tests::{run_tests, TestCase};
use hexium_os::{boot, hlt_loop, init, panic_log};
use hexium_os::{info, print, println}; // RYANS_NOTES: Keeping the imports used in the comments further below
// use crate::{info, print, println};
#[test_case]
fn test_fail_example() {
    println!("test_fail_example...");
    assert_eq!(1, 1);
    println!("ok!");
}

#[test_case]
fn test_example2() {
    println!("test_example2");
    assert_eq!(1+1, 2);
    println!("ok!");
}

fn test_example() -> Result<(), &'static str>{
    assert_eq!(1+1, 2);
    Ok(())
}

// pub fn test_main() {
//     let tests = [
//         TestCase{
//             name: "test_example",
//             function: test_example,
//         }
//     ];

//     run_tests(&tests);
// }

#[cfg(test)]
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    // info!("Test Main");
    assert!(boot::BASE_REVISION.is_supported());
    init();
    info!("Test Main:1");
    test_main();
    info!("Test Main:2");
    loop {}
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(boot::BASE_REVISION.is_supported());

    init();

    info!("Info in main");
    
    panic_log!("Panic log");

    print!("Test2\n");


    #[cfg(test)]
    test_main();

    // RYANS_NOTES: The lines below do not seem to have an effect after the init method above however calling them above the init method causes a bootloop.
    // print!("Test");
    // println!("Test2");
    // info!("Test3");
    panic!("Some message");
    // info!("Test4");

    hlt_loop();
}

// #[cfg(test)]
// #[panic_handler]
// pub fn test_panic(info: &core::panic::PanicInfo) -> ! {
//     hlt_loop();
// }

// #[cfg(not(test))]
// #[panic_handler]
// fn rust_panic(info: &core::panic::PanicInfo) -> ! {
//     use hexium_os::utils::registers::*;
//     panic_log!("{}\n", info);
//     print_register_dump(&get_registers());
//     hlt_loop();
// }

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());

    for test in tests {
        test();
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
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}