#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hexium_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::fmt::Write;

use hexium_os::writer::WRITER;
use core::panic::PanicInfo;
use hexium_os::{boot, hlt_loop, init, panic_log, serial_println, exit_qemu, QemuExitCode, Testable};
use hexium_os::{info, print, println};

#[test_case]
fn test_example() {
    assert_eq!(1+1, 2);
}

#[cfg(test)]
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(boot::BASE_REVISION.is_supported());
    init();
    test_main();
    loop {}
}

#[cfg(not(test))]
#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
    assert!(boot::BASE_REVISION.is_supported());

    /* 
        Issue#30: The lines at the end of this comment below do not seem to have an effect after the init method above 
        however calling them above the init method causes a boot-loop. 
        NOTE: Calling them after the init method after the executor code has been commented back in,
        will cause them not to be run as the executor code seems to block the 'thread'.
        print!("Test");
        println!("Test2");
    */ 

    init();

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