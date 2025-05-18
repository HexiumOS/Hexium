use crate::trace;
use spin::Once;
use x86_64c::{VirtAddr, structures::paging::OffsetPageTable};

pub mod allocator;
pub mod paging;
pub mod pmm;
pub mod vmm;

static mut MAPPER: Option<OffsetPageTable<'static>> = None;
static HHDM_OFFSET: Once<VirtAddr> = Once::new();

pub fn init() {
    if let Some(hhdm_response) = crate::boot::HHDM_REQUEST.get_response() {
        HHDM_OFFSET.call_once(|| VirtAddr::new(hhdm_response.offset()));
    }
    trace!("Hhdm offset: {:#x}", hhdm_offset());

    unsafe {
        MAPPER = Some(vmm::initialize_offset_table());
    }

    #[allow(static_mut_refs)]
    if let Some(offset_page_table) = unsafe { MAPPER.as_mut() } {
        let page_table = offset_page_table;
        match allocator::init_heap(page_table, &mut frame_allocator()) {
            Ok(_) => trace!("Heap initialized"),
            Err(err) => panic!("heap initialization failed: {:?}", err),
        }
    }
    trace!("Memory initialized");
}

/* Get functions */

pub fn hhdm_offset() -> VirtAddr {
    unsafe { *HHDM_OFFSET.get_unchecked() }
}

pub fn frame_allocator() -> pmm::CoreFrameAllocator {
    unsafe { pmm::CoreFrameAllocator::init(crate::boot::MEMMAP_REQUEST.get_response().unwrap()) }
}
