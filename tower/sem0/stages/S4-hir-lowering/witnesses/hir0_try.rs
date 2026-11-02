fn g() -> Result<i32, ()> { Ok(1) }
fn f() -> Result<i32, ()> { let x = g()?; Ok(x) }
fn main() {}
