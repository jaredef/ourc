fn main() { let w = 7; let s = format!("{{{w}}} {0}-{0} {w}|", "z"); println!("{s} {}", s.len()); }
