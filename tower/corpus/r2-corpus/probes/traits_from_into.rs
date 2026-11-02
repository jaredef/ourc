struct Meters(i64);
impl From<i64> for Meters { fn from(x: i64) -> Meters { Meters(x * 3) } }
fn main() { let m: Meters = 4i64.into(); println!("{}", m.0); }
