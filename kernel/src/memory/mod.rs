/*
 * This file is part of Hexium OS.
 * Copyright (C) 2025 The Hexium OS Authors – see the AUTHORS file.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::{boot, trace};
use alloc::{BootInfoFrameAllocator, init_heap};
use spin::once::Once;
use x86_64::structures::paging::OffsetPageTable;
use x86_64::{PhysAddr, VirtAddr, structures::paging::PageTable};

pub mod alloc;
pub mod paging;

static PHYS_MEM_OFFSET: Once<VirtAddr> = Once::new();
static mut MEM_MAPPER: Option<OffsetPageTable<'static>> = None;

pub fn init() {
    if let Some(hhdm_response) = boot::HHDM_REQUEST.get_response() {
        PHYS_MEM_OFFSET.call_once(|| VirtAddr::new(hhdm_response.offset()));
    }
    trace!("Hhdm offset: {:#x}", phys_mem_offset());

    // Create frame allocator
    let mut frame_allocator =
        unsafe { BootInfoFrameAllocator::init(boot::MEMMAP_REQUEST.get_response().unwrap()) };

    unsafe { MEM_MAPPER = Some(paging::init()) };

    // Get a mutable reference to the mapper and initialize heap
    #[allow(static_mut_refs)]
    if let Some(mapper) = unsafe { MEM_MAPPER.as_mut() } {
        init_heap(mapper, &mut frame_allocator).expect("heap initialization failed");
    }
}

pub fn phys_mem_offset() -> VirtAddr {
    unsafe { *PHYS_MEM_OFFSET.get_unchecked() }
}

pub unsafe fn translate_addr(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    translate_addr_inner(addr, physical_memory_offset)
}

fn translate_addr_inner(addr: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    use x86_64::registers::control::Cr3;
    use x86_64::structures::paging::page_table::FrameError;

    // read the active level 4 frame from the CR3 register
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(),
        addr.p3_index(),
        addr.p2_index(),
        addr.p1_index(),
    ];
    let mut frame = level_4_table_frame;

    // traverse the multi-level page table
    for &index in &table_indexes {
        // convert the frame into a page table reference
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe { &*table_ptr };

        // read the page table entry and update `frame`
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // calculate the physical address by adding the page offset
    Some(frame.start_address() + u64::from(addr.page_offset()))
}
