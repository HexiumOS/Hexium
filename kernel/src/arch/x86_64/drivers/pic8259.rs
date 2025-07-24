use crate::{
    arch::io::{inb, iowait, outb},
    trace,
};
use spin::Mutex;

/// First bit (0x01) tells that we want to make use of 4 control words
/// Second bit (0x10) tells that it is the first control word which is getting send
const CMD_ICW1_INIT: u8 = 0x11;
/// Simply just sets the first bit to tell that we use 8086 (x86) mode
const CMD_ICW4_8086_MODE: u8 = 0x01;
/// Signals we wan't to end an interrupt
const CMD_END_OF_INTERRUPT: u8 = 0x20;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub fn init() {
    PICS.lock().initialize();
    trace!("Initialized Legacy PIC");
}

pub static PICS: spin::Mutex<ChainedPics> =
    Mutex::new(ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET));

#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}

struct Pic {
    offset: u8,
    command: u16,
    data: u16,
}

pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: 0x20,
                    data: 0x21,
                },
                Pic {
                    offset: offset2,
                    command: 0xA0,
                    data: 0xA1,
                },
            ],
        }
    }

    pub fn initialize(&mut self) {
        // Control word 1 tells we want to use four ICWs
        outb(self.pics[0].command, CMD_ICW1_INIT);
        iowait();
        outb(self.pics[0].command, CMD_ICW1_INIT);
        iowait();

        // Control Word 2 tells the offsets
        outb(self.pics[0].data, self.pics[0].offset);
        iowait();
        outb(self.pics[1].data, self.pics[1].offset);
        iowait();

        // Control word 3 differs for each PIC
        // Master: Set the bit for the slave PIC’s IR line (IR2 -> set bit 3)
        // Slave: Send the IR line it’s connected to (IR2 -> value 2)
        outb(self.pics[0].data, 0x4); // tell PIC1 a slave is on IRQ2 (0000 0100)
        iowait();
        outb(self.pics[1].data, 0x2); // tell PIC2 its cascade ID (0000 0010)
        iowait();

        // Control word 4 to tell that we are on x86 (8086 mode)
        outb(self.pics[0].data, CMD_ICW4_8086_MODE);
        iowait();
        outb(self.pics[1].data, CMD_ICW4_8086_MODE);
        iowait();

        // CLear data registers
        outb(self.pics[0].data, 0);
        iowait();
        outb(self.pics[1].data, 0);
        iowait();
    }

    pub fn mask(&self, mut irq: u16) {
        let port: u16;
        if irq < 8 {
            port = self.pics[0].data;
        } else {
            irq -= 8;
            port = self.pics[1].data;
        }

        let mask = inb(port);
        outb(port, mask | (1 << irq));
    }

    pub fn unmask(&self, mut irq: u16) {
        let port: u16;
        if irq < 8 {
            port = self.pics[0].data;
        } else {
            irq -= 8;
            port = self.pics[1].data;
        }

        let mask = inb(port);
        outb(port, mask & !(1 << irq));
    }

    pub fn disable(&self) {
        outb(self.pics[0].data, u8::MAX);
        iowait();
        outb(self.pics[1].data, u8::MAX);
        iowait();
    }

    pub fn send_eoi(&self, irq: u32) {
        if irq >= 8 {
            outb(self.pics[1].command, CMD_END_OF_INTERRUPT);
        }
        outb(self.pics[0].command, CMD_END_OF_INTERRUPT);
    }
}
