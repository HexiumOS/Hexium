use limine::memory_map::EntryType;

use crate::{
    arch::{addr::PhysAddr, boot::MEMMAP_REQUEST},
    debug,
    util::create_slice_mut,
};

pub const FRAME_SIZE: u64 = 4096;

pub fn init() {
    create_bitmap_allocator();
}

pub fn create_bitmap_allocator() /* -> BitmapAllocator */
{
    let memmap = MEMMAP_REQUEST.get_response().unwrap();

    let mut high: u64 = 0;
    for entry in memmap.entries() {
        if entry.entry_type == EntryType::USABLE {
            let top: u64 = entry.base + entry.length;
            if top > high {
                high = top;
            }
            debug!(
                "Found usable memory region from: {:#x} to {:#x}",
                entry.base, top
            );
        }
    }
}

pub struct BitmapAllocator {
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
                            start: PhysAddr::new((frame_idx as u64) * FRAME_SIZE),
                        });
                    }
                }
            }
        }
        None
    }

    pub fn free(&mut self, frame: PhysFrame) {
        let addr = frame.start_address().as_u64();

        if addr % FRAME_SIZE != 0 {
            panic!("Unaligned frame address");
        }

        let frame_idx = (addr / FRAME_SIZE) as usize;

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
