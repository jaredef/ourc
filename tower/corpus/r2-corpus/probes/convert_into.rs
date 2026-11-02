struct W(i64);
impl From<i64> for W { fn from(x: i64) -> W { W(x * 2) } }
fn main() {
    let w: W = 21i64.into();
    println!("{}", w.0);
}
