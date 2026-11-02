fn take(f: for<'a> fn(&'a u8) -> &'a u8) -> u8 {
    let x = 9u8;
    *f(&x)
}
fn id(v: &u8) -> &u8 { v }
fn main() {
    println!("{}", take(id));
}
