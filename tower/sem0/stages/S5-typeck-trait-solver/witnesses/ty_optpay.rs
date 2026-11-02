fn f(o: &Option<i32>) -> i32 { match o { Some(x) => *x, None => 0 } }
fn g(r: &Result<i32, bool>) -> i32 { match r { Ok(x) => *x, Err(_e) => 0 } }
fn main() {}
