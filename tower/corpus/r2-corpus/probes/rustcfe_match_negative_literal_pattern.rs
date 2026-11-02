fn tok_prec(t: i32) -> i32 { match t { -1 => 1, -2 => 2, -3 => 0, -4 => 0, _ => if t >= 0 { 9 } else { -1 } } }
fn main() { let ts = vec![5, -1, -2, -3, 7]; let mut s = 0; let mut i = 0; while i < ts.len() { s = s + tok_prec(ts[i]); i = i + 1; } println!("{}", s); }
