pub mod registers;
pub mod types;

pub fn octal_to_binrary(s: &[u8]) -> i32 {
    let mut n = 0;
    for &byte in s {
        n *= 8;
        n += (byte - b'0') as i32;
    }
    n
}
