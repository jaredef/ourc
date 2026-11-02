fn main() { let w = 7; let s = format!("[{:>4}|{:<5}|{:^7}|{:05}|{:8.3}|{w:>3}]", 42, "ab", "mid", -12, 2.5); println!("{} {}", s, s.len()); }
