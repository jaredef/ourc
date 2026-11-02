macro_rules! decl { ($t:ty) => { { let x: $t = 0; x } } }
macro_rules! pair { ($a:ty, $b:ty) => { { let p: (Vec<$a>, $b) = (Vec::new(), 0); p } } }
fn f() -> i64 { decl!(i64) }
fn g() -> (Vec<u8>, i32) { pair!(u8, i32) }
fn main() {}
