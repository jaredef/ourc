fn plain(x: i32) -> bool { matches!(x, 1 | 2) }
fn guarded(x: i32) -> bool { matches!(x, n if n > 0) }
fn main() {}
