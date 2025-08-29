use crate::{
    arch::{
        boot::HHDM_REQUEST,
        memory::pmm::{FRAME_SIZE, alloc_contiguous},
    },
    trace,
};
use core::{mem::MaybeUninit, slice};
use talc::{ErrOnOom, Talc, Talck};

#[global_allocator]
static GLOBAL_ALLOCATOR: Talck<spin::Mutex<()>, ErrOnOom> = Talck::new(Talc::new(ErrOnOom));

pub fn init() {
    let allocator_size = 4 * 0x400 * 0x400;
    let count = (allocator_size / FRAME_SIZE) as usize;
    let allocator_frame = alloc_contiguous(&count);

    let allocator_memory = unsafe {
        let frame = allocator_frame.unwrap();
        let start_addr = frame.start_address();
        slice::from_raw_parts_mut::<MaybeUninit<u8>>(
            (u64::from(HHDM_REQUEST.get_response().unwrap().offset()) + start_addr.as_u64())
                as *mut _,
            allocator_size as usize,
        )
    };

    {
        let mut talc = GLOBAL_ALLOCATOR.lock();
        let span = allocator_memory.into();
        unsafe { talc.claim(span) }.unwrap();
    }

    trace!(
        "Initialized heap memory at {:#x} with size of {:#x}",
        allocator_frame.unwrap().start_address().as_u64(),
        allocator_size
    );
}
