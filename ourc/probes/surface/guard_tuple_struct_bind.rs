struct P { x: i64, y: i64 }
fn main() { for p in [(1, 1), (2, -2), (5, 1), (-4, 1)] { let s = match p { (a, b) if a == b => "eq", (a, b) if a + b == 0 => "neg", (a, _) if a > 0 => "pos", _ => "else" }; print!("{} ", s); }
  let q = P { x: 2, y: 2 }; let t = match q { P { x, y } if x == y => "diag", P { x, .. } if x > 0 => "right", _ => "o" }; println!("{}", t); }
