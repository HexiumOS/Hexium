use crate::hal::clock::DateTime;

pub mod rtc;

pub unsafe fn read() -> DateTime {
    unsafe { rtc::read_rtc() }
}
