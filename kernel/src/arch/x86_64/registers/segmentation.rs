pub trait Segment {
    fn get_reg() -> SegmentSelector;
    unsafe fn set_reg(sel: SegmentSelector);
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct SegmentSelector(pub u16);

impl SegmentSelector {
    #[inline]
    pub const fn new(index: u16, rpl: super::super::PrivilegeLevel) -> SegmentSelector {
        SegmentSelector((index << 3) | (rpl as u16))
    }
}

pub struct CS;
pub struct SS;
