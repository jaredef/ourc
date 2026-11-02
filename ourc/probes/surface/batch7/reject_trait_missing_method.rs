trait T { fn f(&self) -> i64; } struct S; impl T for S {} fn main() { println!("{}", S.f()); }
