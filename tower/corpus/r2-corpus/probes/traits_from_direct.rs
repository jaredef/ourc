struct W(i64);
impl From<i64> for W { fn from(x: i64) -> W { W(x + 10) } }
fn main() { let w = W::from(5); println!("{}", w.0); }
