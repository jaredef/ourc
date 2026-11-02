enum E { A(i32), B(i32) }
fn main() {
    let n = Some(3);
    if let Some(1 | 3) = n { println!("13"); }
    if let Some(2 | 4) = n { println!("no"); } else { println!("else"); }
    let e = E::B(7);
    let r = if let E::A(1) | E::B(7) = e { 1 } else { 0 };
    if let Some(x) = n { println!("plain {}", x); }
    println!("{}", r);
}
