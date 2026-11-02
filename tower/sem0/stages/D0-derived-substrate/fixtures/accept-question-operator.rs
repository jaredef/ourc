fn f() -> Option<i32> { let v: Option<i32> = Some(1); Some(v? + 1) }
fn main(){ println!("{:?}", f()); }
