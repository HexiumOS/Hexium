#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hexium_os::test_runner)]
#![reexport_test_harness_main="test_main"]

use core::panic::PanicInfo;

use hexium_os::{println, init};
// use hexium_os::writer::*;

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    init(); // RYAN_NOTES: Not sure why it's absence causes an loop running of test_println_long test.
  test_main();
  loop{}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  hexium_os::test_panic_handler(info);
  loop{}
}

#[test_case]
fn test_println_simple() {
    println!("Simple print new line statement");
}

#[test_case]
fn test_println_long() {
    for _ in 0..200 {
        println!("Simple print new line many times");
    }
}

#[test_case]
fn test_println_long_more() {
    for _ in 0..200 {
        println!("Simple print new line many times");
    }
}