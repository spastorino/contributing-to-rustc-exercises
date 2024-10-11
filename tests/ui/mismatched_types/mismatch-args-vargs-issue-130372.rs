#![feature(c_variadic)]


unsafe extern "C" fn test_va_copy(_: u64, mut ap: ...) {}

pub fn main() {
    unsafe {
        test_va_copy();
    }
}

