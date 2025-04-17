#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main="test_main"]

use core::panic::PanicInfo;

use hexium_os::{exit_qemu, init, serial_print, serial_println};

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
  init();
  test_main();
  loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  serial_println!("[ok]");
  exit_qemu(hexium_os::QemuExitCode::Success);
  loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
  serial_println!("Running {} tests", tests.len());

  for test in tests {
    test();
    serial_print!("[test did not panic]");
    exit_qemu(hexium_os::QemuExitCode::Failed);
  }
  exit_qemu(hexium_os::QemuExitCode::Success);
}

#[test_case]
fn should_fail() {
  serial_println!("should_panic::should_fail...\t");
  assert_eq!(0,1);
}