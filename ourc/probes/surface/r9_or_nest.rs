enum E { A(i32), B(i32), C }
fn k(e: &E) -> &'static str {
    match e {
        E::A(1 | 2) | E::B(2) => "hit",
        E::A(_) | E::B(_) => "other",
        E::C => "c",
    }
}
fn main() {
    let xs = [E::A(1), E::A(5), E::B(2), E::B(3), E::C];
    for x in xs.iter() { print!("{} ", k(x)); }
    let t = match (5, 6) { (1 | 5, 6 | 7) => "x", _ => "y" };
    let o = match Some(3) { Some(1) | Some(3) => 13, _ => 0 };
    println!("{} {}", t, o);
}
