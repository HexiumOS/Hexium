#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use hexium_os::{boot, hlt_loop, init, panic_log};
use hexium_os::{info, print, println}; // RYANS_NOTES: Keeping the imports used in the comments further below
// use crate::{info, print, println};

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
}