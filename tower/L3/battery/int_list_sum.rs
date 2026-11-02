enum List { Cons(i32, i32), Nil }
fn main() { let a = [3, 1, 4, 1, 5]; let mut s = 0; let mut i = 0; while i < 5 { s = s + a[i as usize]; i = i + 1; } std::process::exit(s); }