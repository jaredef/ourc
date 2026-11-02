struct Cel(i64);
impl From<i64> for Cel { fn from(x: i64) -> Cel { Cel(x + 1) } }
fn main() {
    let c: Cel = Cel::from(41);
    println!("{}", c.0);
}
