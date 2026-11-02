const SIZE: usize = 4;
static SQUARES: [i64; SIZE] = [0, 1, 4, 9];
fn main() {
    let mut s = 0i64; let mut i = 0;
    while i < SIZE { s += SQUARES[i]; i += 1; }
    println!("{}", s);
}
