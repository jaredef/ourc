pub fn error_string(errno: i32) -> u32 {
    extern "C" {
        fn strerror_r(errnum: i32, buf: *mut u8, buflen: usize) -> i32;
    }
    0
}
fn main() {}
