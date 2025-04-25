#![no_std]
#![no_main]

use core::panic::PanicInfo;

use hexium_os::{exit_qemu, init, serial_print, serial_println};

#[unsafe(no_mangle)]
unsafe extern "C" fn kmain() -> ! {
  init();
  should_fail();
  serial_print!("[test did not panic]");
  exit_qemu(hexium_os::QemuExitCode::Failed);
  loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  serial_println!("[ok]");
  exit_qemu(hexium_os::QemuExitCode::Success);
  loop {}
}

fn should_fail() {
  serial_println!("should_panic::should_fail...\t");
  assert_eq!(0,1);
}