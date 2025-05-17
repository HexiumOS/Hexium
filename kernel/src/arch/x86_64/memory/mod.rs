use crate::trace;
use spin::Once;
use x86_64c::{VirtAddr, structures::paging::OffsetPageTable};

pub mod paging;
pub mod pmm;
pub mod vmm;

static mut MAPPER: Once<OffsetPageTable<'static>> = Once::new();
static HHDM_OFFSET: Once<VirtAddr> = Once::new();

pub fn init() {
    if let Some(hhdm_response) = crate::boot::HHDM_REQUEST.get_response() {
        HHDM_OFFSET.call_once(|| VirtAddr::new(hhdm_response.offset()));
    }
    trace!("Hhdm offset: {:#x}", hhdm_offset());
}

pub fn hhdm_offset() -> VirtAddr {
    unsafe { *HHDM_OFFSET.get_unchecked() }
}

pub fn mapper() -> &'static OffsetPageTable<'static> {
    #[allow(static_mut_refs)]
    unsafe {
        MAPPER.get_unchecked()
    }
}
