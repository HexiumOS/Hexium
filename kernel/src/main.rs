#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hexium_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::fmt::Write;

use hexium_os::writer::WRITER;
use core::panic::PanicInfo;
use hexium_os::{boot, hlt_loop, init, panic_log, serial_println, exit_qemu, QemuExitCode, Testable};
use hexium_os::{info, print, println}; // RYANS_NOTES: Keeping the imports used in the comments further below
// use crate::{info, print, println};

#[test_case]
fn test_fail_example() {
    assert_eq!(0, 1);
}

#[test_case]
fn test_example() {
    assert_eq!(1+1, 2);
}

// RYAN_NOTES: Refactor these tests out of here
// #[test_case]
// fn test_println_simple() {
//     println!("Simple print new line statement");
// }

// #[test_case]
// fn test_println_long() {
//     for _ in 0..200 {
//         println!("Simple print new line many times");
//     }
// }

// TODO: RYAN_NOTES: Needs buffer access
// #[test_case]
// fn test_println_output() {
//     let s = "Some test fitting single line";
//     println!("{}", s);
//     for (i, c) in s.chars().enumerate() {
//         let screen_char = WRITER.lock().write_char(c).buffer.chars[BUFFER_HEIGHT - 2][i].read();
//         assert_eq!(char::from(screen_char.ascii_character), c);
//     }
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

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use hexium_os::utils::registers::{print_register_dump, get_registers};
    panic_log!("{}\n", info);
    print_register_dump(&get_registers());
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hexium_os::test_panic_handler(info)
}