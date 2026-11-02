mod a {
    pub mod b {
        pub fn f() -> i32 { 7 }
        pub fn g() -> i32 { 8 }
    }
    pub fn h() -> i32 { 9 }
}
use a::b::f as ff;
use a::b::g as gg;
use a::h;
use a::h as hh;
fn main() {
    println!("{} {} {} {}", ff(), gg(), h(), hh());
}
