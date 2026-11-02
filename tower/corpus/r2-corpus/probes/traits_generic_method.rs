trait Conv { fn conv<T: From<i64>>(&self) -> T; }
struct S;
impl Conv for S { fn conv<T: From<i64>>(&self) -> T { T::from(8) } }
fn main() { let x: i64 = S.conv(); println!("{}", x); }
