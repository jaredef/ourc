fn f(t: &str) -> bool { let b = t.as_bytes(); let n = b.len(); let mut i = 0; let mut prev = 0; while i < n { if prev != 58 { return true; } prev = b[i]; i = i + 1; } false }
fn main(){}
