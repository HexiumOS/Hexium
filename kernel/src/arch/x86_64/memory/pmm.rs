use core::ops::Range;

use limine::{
    memory_map::{Entry, EntryType},
    response::MemoryMapResponse,
};
use x86_64c::{
    PhysAddr,
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
};

/// A FrameAllocator that always returns `None`.
pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

/// A FrameAllocator that returns usable frames from the bootloader's memory map.
pub struct CoreFrameAllocator {
    memory_map: &'static MemoryMapResponse,
    next: usize,
}

impl CoreFrameAllocator {
    /// Create a FrameAllocator from the passed memory map.
    pub unsafe fn init(memory_map: &'static MemoryMapResponse) -> Self {
        CoreFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    /// Returns an iterator over the usable frames specified in the memory map.
    pub fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        // get usable regions from memory map
        let regions: &[&Entry] = self.memory_map.entries();
        let usable_regions = regions
            .into_iter()
            .filter(|r: &&&Entry| r.entry_type == EntryType::USABLE);
        // map each region to its address range
        let addr_ranges = usable_regions.map(|r: &&Entry| r.base..(r.base + r.length));
        // transform to an iterator of frame start addresses
        let frame_addresses = addr_ranges.flat_map(|r: Range<u64>| r.step_by(4096));
        // create `PhysFrame` types from the start addresses
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for CoreFrameAllocator {
    /// Returns the next usable frame, or `None` if exhausted.
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}
