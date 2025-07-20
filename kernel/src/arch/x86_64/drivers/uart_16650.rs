use crate::arch::x86_shared as x86;

const SERIAL_PORT1: u16 = 0x3F8; // Also known as COM1

pub fn init() {
    x86::io::outb(interrupt_port(), 0x00);

    // Enable DLAB to configure speed (bits per second)
    x86::io::outb(line_control_port(), 0x80);

    // Set maximum speed to 38400 bps by configuring DLL and DLM
    x86::io::outb(data_port(), 0x03);
    x86::io::outb(interrupt_port(), 0x00);

    // Disable DLAB and set data word length to 8 bits
    x86::io::outb(line_control_port(), 0x03);

    // Enable FIFO, clear TX/RX queues and
    // set interrupt watermark at 14 bytes
    x86::io::outb(fifo_control_port(), 0xc7);

    // Mark data terminal ready, signal request to send
    // and enable auxilliary output #2 (used as interrupt line for CPU)
    x86::io::outb(modem_control_port(), 0x0b);

    x86::io::outb(interrupt_port(), 0x01);
}

pub fn send(data: u8) {
    match data {
        8 | 0x7F => {
            send_raw(8);
            send_raw(b' ');
            send_raw(8);
        }
        data => {
            send_raw(data);
        }
    }
}

fn send_raw(data: u8) {
    crate::retry_until_ok!(try_send_raw(data))
}

fn try_send_raw(data: u8) -> Result<(), WouldBlockError> {
    if line_status().contains(LineStatusFlags::OUTPUT_EMPTY) {
        x86::io::outb(data_port(), data);
        Ok(())
    } else {
        Err(WouldBlockError)
    }
}

fn line_status() -> LineStatusFlags {
    LineStatusFlags::from_bits_truncate(x86::io::inb(line_status_port()))
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct LineStatusFlags: u8 {
        const INPUT_FULL = 1;
        const OUTPUT_EMPTY = 1 << 5;
    }
}

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WouldBlockError;

fn base_port() -> u16 {
    SERIAL_PORT1
}

fn data_port() -> u16 {
    base_port()
}

fn interrupt_port() -> u16 {
    base_port() + 1
}

fn fifo_control_port() -> u16 {
    base_port() + 2
}

fn line_control_port() -> u16 {
    base_port() + 3
}

fn modem_control_port() -> u16 {
    base_port() + 4
}

fn line_status_port() -> u16 {
    base_port() + 5
}
