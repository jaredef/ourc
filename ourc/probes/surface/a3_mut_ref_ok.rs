fn bump(v: &mut i32) { *v += 1; }
fn main() {
    let mut x = 1;
    let y = &mut x;
    *y += 1;
    bump(&mut x);
    println!("{}", x);
}
