use std::fmt; struct Pt { x: i64, y: i64 } impl fmt::Display for Pt { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({}, {})", self.x, self.y) } }
fn main() { let p = Pt { x: 1, y: -2 }; let s = p.to_string(); println!("{} [{}] {} {} {} {}", p, s, s.len(), true.to_string(), 5.to_string(), (3 > 9).to_string()); }
