enum LK { Int { base: i64, n: i64 }, Unit }
use LK::*;
fn mk(base: i64, x: char) -> LK {
    match x { 'u' => Unit, _ => return Int { base, n: base * 2 } }
}
fn main() {
    let k = mk(7, 'z');
    let r = match k { Int { base, n } => base + n, Unit => 0 };
    println!("{}", r);
}
