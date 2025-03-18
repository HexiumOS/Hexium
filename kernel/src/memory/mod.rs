use crate::{boot, trace};
use spin::once::Once;
use x86_64::VirtAddr;

pub mod paging;

static PHYS_MEM_OFFSET: Once<VirtAddr> = Once::new();

pub fn init() {
    if let Some(hhdm_response) = boot::HHDM_REQUEST.get_response() {
        PHYS_MEM_OFFSET.call_once(|| VirtAddr::new(hhdm_response.offset()));
    }
    trace!("Hhdm offset: {:#x}\n", phys_mem_offset());
}

pub fn phys_mem_offset() -> VirtAddr {
    unsafe { *PHYS_MEM_OFFSET.get_unchecked() }
}
