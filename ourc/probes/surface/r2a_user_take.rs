// R2a P4 guard: a user `fn take(&mut self)` keeps its own method (OPT-TAKE stands down when the program defines `fn take`).
struct C { n: i64 }
impl C { fn take(&mut self) -> i64 { let t = self.n; self.n = 0; t } }
fn main() { let mut c = C { n: 9 }; println!("{} {}", c.take(), c.n); }
