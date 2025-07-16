use crate::trace;
use lazy_static::lazy_static;
use x86_64c::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

lazy_static! {
    pub static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&super::tss::TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
                data_selector,
            },
        )
    };
}

pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
    pub data_selector: SegmentSelector,
}

pub fn init() {
    use x86_64c::instructions::segmentation::{CS, SS, Segment};

    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        SS::set_reg(GDT.1.data_selector);
    }

    trace!("initialized GDT");
}
