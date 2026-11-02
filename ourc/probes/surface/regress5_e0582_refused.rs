fn bar<F>(t: F) where F: for<'a> Fn() -> Option<&'a i32> {}
fn main() {}
