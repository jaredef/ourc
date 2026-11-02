mod a { pub fn f() -> i64 { 1 } pub mod b { pub fn g() -> i64 { super::f() + 1 } } } mod c { pub fn h() -> i64 { crate::a::f() + 2 } } use a::b::g; fn main(){ println!("{} {}", g(), c::h()); }
