enum E { N(i32) }
fn main() {
    let v = E::N(4);
    let r = match v { E::N(x @ 1..=10) => x*3, E::N(_) => 0 };
    println!("{}", r);
}
