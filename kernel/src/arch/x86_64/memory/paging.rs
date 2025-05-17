use x86_64c::structures::paging::PageTable;

/// Returns a mutable reference to the active level 4 table.
pub unsafe fn active_level_4_table() -> &'static mut PageTable {
    use x86_64c::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = super::hhdm_offset() + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}
