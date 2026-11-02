fn parse_num(s: &str) -> Option<i32> { let mut n = 0; let mut any = false; for b in s.bytes() { if b >= 48 && b <= 57 { n = n * 10 + (b as i32 - 48); any = true; } else { return None; } } if any { Some(n) } else { None } }
fn total(strs: &[&str]) -> Option<i32> { let mut t = 0; let mut i = 0; while i < strs.len() { let v = parse_num(strs[i])?; t = t + v; i = i + 1; } Some(t) }
fn main() { let ss = vec!["12", "30", "8"]; match total(&ss) { Some(n) => println!("{}", n), None => println!("bad") } }
