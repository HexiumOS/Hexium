/*
 * This file is part of Hexium OS.
 * Copyright (C) 2025 The Hexium OS Authors – see the AUTHORS file.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::hal::clock::DateTime;
use crate::print;
use core::fmt;

const RTC_ADDRESS_PORT: u16 = 0x70;
const RTC_DATA_PORT: u16 = 0x71;

const RTC_SECONDS: u8 = 0x00;
const RTC_MINUTES: u8 = 0x02;
const RTC_HOURS: u8 = 0x04;
const RTC_DAY: u8 = 0x07;
const RTC_MONTH: u8 = 0x08;
const RTC_YEAR: u8 = 0x09;

/// Implement Display trait for DateTime to format it nicely
impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}/{:02}/{:02} {:02}:{:02}:{:02}",
            self.day, self.month, self.year, self.hour, self.minute, self.second
        )
    }
}

/// Function to read a byte from the RTC
unsafe fn rtc_read(register: u8) -> u8 {
    use crate::arch::io;
    unsafe { io::outb(RTC_ADDRESS_PORT, register) };
    unsafe { io::inb(RTC_DATA_PORT) }
}

/// Function to convert BCD to binary
fn bcd_to_bin(bcd: u8) -> u8 {
    ((bcd >> 4) * 10) + (bcd & 0x0F)
}

/// Function to read the current date and time from the RTC
pub unsafe fn read_rtc() -> DateTime {
    let second = bcd_to_bin(unsafe { rtc_read(RTC_SECONDS) });
    let minute = bcd_to_bin(unsafe { rtc_read(RTC_MINUTES) });
    let hour = bcd_to_bin(unsafe { rtc_read(RTC_HOURS) });
    let day = bcd_to_bin(unsafe { rtc_read(RTC_DAY) });
    let month = bcd_to_bin(unsafe { rtc_read(RTC_MONTH) });
    let year = bcd_to_bin(unsafe { rtc_read(RTC_YEAR) });

    DateTime {
        second,
        minute,
        hour,
        day,
        month,
        year,
    }
}

pub fn print_date_time() {
    let date_time = unsafe { read_rtc() };
    print!("Current Date and Time: {}", date_time);
}
