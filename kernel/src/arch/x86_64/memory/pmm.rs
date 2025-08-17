use crate::arch::{
    addr::PhysAddr,
    boot::{HHDM_REQUEST, MEMMAP_REQUEST},
};

pub const FRAME_SIZE: u64 = 4096;

pub struct BitmapAllocator {
    base_addr: PhysAddr,
    frame_count: usize,
    bitmap: &'static mut [u64],
}

impl BitmapAllocator {
    pub fn alloc(&mut self) -> Option<PhysFrame> {
        for (word_idx, word) in self.bitmap.iter_mut().enumerate() {
            if *word != u64::MAX {
                for bit_idx in 0..64 {
                    let mask = 1u64 << bit_idx;
                    if *word & mask == 0 {
                        *word |= mask;
                        let frame_idx = word_idx * 64 + bit_idx as usize;
                        if frame_idx >= self.frame_count {
                            return None;
                        }
                        return Some(PhysFrame {
                            start: PhysAddr::new(
                                self.base_addr.as_u64() + (frame_idx as u64) * FRAME_SIZE,
                            ),
                        });
                    }
                }
            }
        }
        None
    }

    pub fn free(&mut self, frame: PhysFrame) {
        let addr = frame.start_address();
        if addr.as_u64() < self.base_addr.as_u64() {
            panic!("Frame address below base");
        }
        let offset = addr.as_u64() - self.base_addr.as_u64();
        if offset % FRAME_SIZE != 0 {
            panic!("Unaligned frame address");
        }
        let frame_idx = (offset / FRAME_SIZE) as usize;
        if frame_idx >= self.frame_count {
            panic!("Frame index out of bounds");
        }
        let word_idx = frame_idx / 64;
        let bit_idx = (frame_idx % 64) as u32;
        if word_idx >= self.bitmap.len() {
            panic!("Bitmap index out of bounds");
        }
        let mask = 1u64 << bit_idx;
        if self.bitmap[word_idx] & mask == 0 {
            panic!("Frame already free");
        }
        self.bitmap[word_idx] &= !mask;
    }
}

#[derive(Clone, Copy)]
pub struct PhysFrame {
    start: PhysAddr,
}

impl PhysFrame {
    pub fn containing_address(addr: PhysAddr) -> Self {
        let aligned_addr = PhysAddr::new(addr.as_u64() & !(FRAME_SIZE - 1));
        Self {
            start: aligned_addr,
        }
    }

    pub fn start_address(&self) -> PhysAddr {
        self.start
    }
}
