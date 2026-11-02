#[derive(Clone, Copy)]
enum Lvl { Low, Mid = 5, High, Neg = -2, After }
enum Plain { X, Y, Z }
fn val(l: Lvl) -> i64 { l as i64 * 10 }
fn main() {
    let v = Lvl::High;
    let n = Lvl::Neg;
    println!("{} {} {} {} {} {}", v as i64, n as i32, Lvl::After as i64, Plain::Z as u8, Plain::X as usize, val(Lvl::Mid));
    match v { Lvl::High => println!("high"), _ => println!("other") }
    let _ = Plain::Y;
}
