struct S { x: i64 }
fn main() {
    let mut s = S { x: 1 };
    s.x = s.x + 40;
    s.x += 1;
    println!("{}", s.x);
}
