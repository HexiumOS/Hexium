use x86_64::instructions::port::Port;

const CMD_INIT: u8 = 0x11;

const CMD_END_OF_INTERRUPT: u8 = 0x20;

const MODE_8086: u8 = 0x01;

struct Pic {
    offset: u8,
    command: Port<u8>,
    data: Port<u8>,
}

impl Pic {
    fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.offset <= interrupt_id && interrupt_id < self.offset + 8
    }

    unsafe fn end_of_interrupt(&mut self) { unsafe {
        self.command.write(CMD_END_OF_INTERRUPT);
    }}

    unsafe fn read_mask(&mut self) -> u8 { unsafe {
        self.data.read()
    }}

    unsafe fn write_mask(&mut self, mask: u8) { unsafe {
        self.data.write(mask)
    }}
}

pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: Port::new(0x20),
                    data: Port::new(0x21),
                },
                Pic {
                    offset: offset2,
                    command: Port::new(0xA0),
                    data: Port::new(0xA1),
                },
            ],
        }
    }

    pub const unsafe fn new_contiguous(primary_offset: u8) -> ChainedPics { unsafe {
        Self::new(primary_offset, primary_offset + 8)
    }}

    pub unsafe fn initialize(&mut self) { unsafe {
        let mut wait_port: Port<u8> = Port::new(0x80);
        let mut wait = || wait_port.write(0);

        self.pics[0].command.write(CMD_INIT);
        wait();
        self.pics[1].command.write(CMD_INIT);
        wait();

        self.pics[0].data.write(self.pics[0].offset);
        wait();
        self.pics[1].data.write(self.pics[1].offset);
        wait();

        self.pics[0].data.write(4);
        wait();
        self.pics[1].data.write(2);
        wait();

        self.pics[0].data.write(MODE_8086);
        wait();
        self.pics[1].data.write(MODE_8086);
        wait();

        // Mask everything exept the PIT
        self.write_masks(0xFE, 0xFF);
    }}

    pub unsafe fn read_masks(&mut self) -> [u8; 2] { unsafe {
        [self.pics[0].read_mask(), self.pics[1].read_mask()]
    }}

    pub unsafe fn write_masks(&mut self, mask1: u8, mask2: u8) { unsafe {
        self.pics[0].write_mask(mask1);
        self.pics[1].write_mask(mask2);
    }}

    pub unsafe fn disable(&mut self) { unsafe {
        self.write_masks(u8::MAX, u8::MAX)
    }}

    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) { unsafe {
        if self.handles_interrupt(interrupt_id) {
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt();
            }
            self.pics[0].end_of_interrupt();
        }
    }}
}
