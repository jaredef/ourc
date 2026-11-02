fn main() { let mut s = String::new(); for i in 0..3 { s.push_str(&i.to_string()); s.push('-'); } println!("{} {}", s, s.trim_end_matches('-')); }
