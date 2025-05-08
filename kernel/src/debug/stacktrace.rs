use crate::print;

#[repr(C)]
struct StackFrame {
    prev_frame: *const StackFrame,
    return_address: usize,
}

pub unsafe fn print_stack_trace() {
    crate::println!("\x1b[37;41mStacktrace:");

    let mut frame: *const StackFrame;

    unsafe {
        core::arch::asm!(
            "mov {}, rbp",
            out(reg) frame,
            options(nostack, nomem, preserves_flags)
        );
    }

    let mut depth = 0;

    while let Some(f) = unsafe { frame.as_ref() } {
        print!(
            "Frame {}: return address = 0x{:016X}\n",
            depth, f.return_address
        );
        frame = f.prev_frame;
        depth += 1;

        if depth > 64 {
            print!("Stack trace aborted (too deep)\n");
            break;
        }
    }
}
