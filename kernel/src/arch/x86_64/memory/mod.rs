use crate::trace;
use spin::Once;
use x86_64c::VirtAddr;

pub mod paging;
pub mod pmm;
pub mod vmm;

static PHYS_MEM_OFFSET: Once<VirtAddr> = Once::new();

pub fn init() {
    if let Some(hhdm_response) = crate::boot::HHDM_REQUEST.get_response() {
        PHYS_MEM_OFFSET.call_once(|| VirtAddr::new(hhdm_response.offset()));
    }
    trace!("Hhdm offset: {:#x}", hhdm_offset());
}

pub fn hhdm_offset() -> VirtAddr {
    unsafe { *PHYS_MEM_OFFSET.get_unchecked() }
}
