fn malloc(x: i64) -> i64 { x + 1 }
fn printf(x: i64) -> i64 { x * 2 }
fn strlen(x: i64) -> i64 { x - 1 }
fn exit(x: i64) -> i64 { x + 10 }
fn main() { let v = vec![1, 2]; let s = format!("{}-{}", "a", v.len()); println!("{} {} {} {} {} {}", malloc(41), printf(4), strlen(9), exit(0), s, s.len()); }
