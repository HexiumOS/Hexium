use crate::hal::clock::DateTime;

pub mod pit;
pub mod rtc;

pub unsafe fn read_clock() -> DateTime {
    unsafe { rtc::read_rtc() }
}
