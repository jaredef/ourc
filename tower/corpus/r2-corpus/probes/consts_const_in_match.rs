const LO: i64 = 1;
const HI: i64 = 9;
fn main() {
    let x = 9;
    let r = match x { LO => 100, HI => 200, _ => 0 };
    println!("{}", r);
}
