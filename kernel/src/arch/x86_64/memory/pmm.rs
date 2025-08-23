use crate::{
    arch::{
        addr::PhysAddr,
        boot::{HHDM_REQUEST, MEMMAP_REQUEST},
    },
    debug, trace,
};
use limine::memory_map::EntryType;

pub const FRAME_SIZE: u64 = 4096;

static mut BITMAP_ALLOCATOR: Option<BitmapAllocator> = None;

pub fn init() {
    unsafe {
        BITMAP_ALLOCATOR = Some(create_bitmap_allocator());
    }
}

pub fn create_bitmap_allocator() -> BitmapAllocator {
    let memmap = MEMMAP_REQUEST.get_response().unwrap();
    let hhdm = HHDM_REQUEST.get_response().unwrap().offset();

    // Find the frame count and bitmap size
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

    let frame_count = (high / FRAME_SIZE) as usize;
    let bitmap_size_bytes = (frame_count + 7) / 8; // Round up to nearest byte
    let bitmap_size_u64s = (bitmap_size_bytes + 7) / 8; // Round up to nearest u64

    debug!(
        "Total frames: {}, bitmap size: {} bytes ({} u64s)",
        frame_count, bitmap_size_bytes, bitmap_size_u64s
    );

    // Find a suitable location for the bitmap
    // Look for the largest usable region that can fit our bitmap
    let mut best_region: Option<(u64, u64)> = None;
    let mut best_size = 0u64;

    for entry in memmap.entries() {
        if entry.entry_type == EntryType::USABLE && entry.length >= bitmap_size_bytes as u64 {
            if entry.length > best_size {
                best_size = entry.length;
                best_region = Some((entry.base, entry.length));
            }
        }
    }

    let (bitmap_base, _) = best_region.expect("No suitable memory region found for bitmap");

    debug!("Placing bitmap at physical address: {:#x}", bitmap_base);

    // Create the bitmap slice from the chosen memory region
    let bitmap_ptr = (bitmap_base + hhdm) as *mut u64;
    let bitmap = unsafe { core::slice::from_raw_parts_mut(bitmap_ptr, bitmap_size_u64s) };

    // Create the bitmap with all frames used (set all bits to 1)
    for word in bitmap.iter_mut() {
        *word = u64::MAX;
    }

    // Free all usable frame entries
    for entry in memmap.entries() {
        if entry.entry_type == EntryType::USABLE {
            let start_frame = entry.base / FRAME_SIZE;
            let end_frame = (entry.base + entry.length) / FRAME_SIZE;

            for frame_idx in start_frame..end_frame {
                if (frame_idx as usize) < frame_count {
                    let word_idx = (frame_idx as usize) / 64;
                    let bit_idx = (frame_idx as usize) % 64;

                    if word_idx < bitmap.len() {
                        let mask = 1u64 << bit_idx;
                        bitmap[word_idx] &= !mask;
                    }
                }
            }

            debug!("Freed frames {:#x} to {:#x}", start_frame, end_frame);
        }
    }

    // Mark the bitmap region itself as used to prevent allocation over it
    let bitmap_start_frame = bitmap_base / FRAME_SIZE;
    let bitmap_end_frame = (bitmap_base + bitmap_size_bytes as u64 + FRAME_SIZE - 1) / FRAME_SIZE;

    for frame_idx in bitmap_start_frame..bitmap_end_frame {
        if (frame_idx as usize) < frame_count {
            let word_idx = (frame_idx as usize) / 64;
            let bit_idx = (frame_idx as usize) % 64;

            if word_idx < bitmap.len() {
                let mask = 1u64 << bit_idx;
                bitmap[word_idx] |= mask; // Set bit to mark as used
            }
        }
    }

    debug!(
        "Marked bitmap region frames {:#x} to {:#x} as used",
        bitmap_start_frame, bitmap_end_frame
    );

    // Print memory information
    let total_memory = frame_count as u64 * FRAME_SIZE;
    let mut free_frames = 0usize;
    for (_word_idx, word) in bitmap.iter().enumerate() {
        let mut w = !*word;
        while w != 0 {
            free_frames += (w & 1) as usize;
            w >>= 1;
        }
    }
    let free_memory = free_frames as u64 * FRAME_SIZE;

    trace!(
        "Physical memory: total = {} MiB, free = {} MiB ({} frames free)",
        total_memory / 1024 / 1024,
        free_memory / 1024 / 1024,
        free_frames
    );

    // Create the allocator after all bitmap initialization is complete
    BitmapAllocator {
        frame_count,
        bitmap,
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
