#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(hexium_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
#[cfg(not(test))]
use hexium_os::{boot, hlt_loop, init, panic_log};
#[cfg(test)]
use hexium_os::{boot, init};

#[test_case]
fn test_example() {
    assert_eq!(1 + 1, 2);
}

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

    #[cfg(test)]
    {
        test_main();
    }

    #[cfg(not(test))]
    hlt_loop();
    #[cfg(test)]
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
// Handles panics in production with detailed register dump
fn panic(info: &PanicInfo) -> ! {
    use hexium_os::utils::registers::{get_registers, print_register_dump};
    panic_log!("{}\n", info);
    print_register_dump(&get_registers());
    loop {}
}

#[cfg(test)]
#[panic_handler]
// Handles panics during binary tests, delegates to test_panic_handler
fn panic(info: &PanicInfo) -> ! {
    hexium_os::test_panic_handler(info)
}
